use serde::{Deserialize, Serialize};
use std::{collections::BTreeMap, fs::{self, OpenOptions}, io::{Read, Write}, path::{Path, PathBuf}, time::{Duration, Instant}};

const LIMIT: usize = 32_768;
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(tag = "action", rename_all = "snake_case", deny_unknown_fields)]
pub enum Action { Read { path: String }, Create { path: String, content: String } }
#[derive(Clone, Debug, Serialize)]
pub struct Proposal { pub id: u64, pub action: Action, pub expires_in_seconds: u64 }
#[derive(Clone, Debug, Serialize)]
pub struct Event { pub id: u64, pub outcome: String }
pub struct Broker { root: PathBuf, next: u64, pending: BTreeMap<u64, (Action, Instant)>, events: Vec<Event> }
impl Broker {
    pub fn new(root: impl AsRef<Path>) -> Result<Self, String> {
        fs::create_dir_all(&root).map_err(|e| e.to_string())?;
        let root = fs::canonicalize(root).map_err(|e| e.to_string())?;
        Ok(Self { root, next: 1, pending: BTreeMap::new(), events: vec![] })
    }
    fn target(&self, action: &Action) -> Result<PathBuf, String> {
        let path = match action { Action::Read { path } | Action::Create { path, .. } => path };
        // Flat ASCII workspace excludes traversal, ADS, device names and nested symlinks.
        if path.is_empty() || path.len() > 80 || !path.ends_with(".txt") ||
            !path.bytes().all(|c| c.is_ascii_alphanumeric() || b"-_.".contains(&c)) || path.contains("..") {
            return Err("Use a simple .txt filename, with no directories or traversal".into());
        }
        let stem = path.split('.').next().unwrap().to_ascii_uppercase();
        if ["CON", "PRN", "AUX", "NUL", "COM1", "COM2", "COM3", "COM4", "COM5", "COM6", "COM7", "COM8", "COM9", "LPT1", "LPT2", "LPT3", "LPT4", "LPT5", "LPT6", "LPT7", "LPT8", "LPT9"].contains(&stem.as_str()) {
            return Err("Reserved device filename".into());
        }
        let target = self.root.join(path);
        if let Ok(meta) = fs::symlink_metadata(&target) {
            if meta.file_type().is_symlink() || !meta.is_file() { return Err("Links and non-files are denied".into()); }
        }
        if let Action::Create { content, .. } = action {
            if content.len() > LIMIT { return Err("Content exceeds 32 KiB".into()); }
            if target.exists() { return Err("Overwrite denied; choose a new filename".into()); }
        }
        Ok(target)
    }
    pub fn propose(&mut self, action: Action) -> Result<Proposal, String> {
        self.target(&action)?;
        self.pending.retain(|_, (_, at)| at.elapsed() < Duration::from_secs(120));
        if self.pending.len() >= 32 { return Err("Too many pending proposals".into()); }
        let id = self.next; self.next += 1;
        self.pending.insert(id, (action.clone(), Instant::now()));
        Ok(Proposal { id, action, expires_in_seconds: 120 })
    }
    pub fn decide(&mut self, id: u64, approve: bool) -> Result<String, String> {
        let (action, at) = self.pending.remove(&id).ok_or("Unknown or already consumed proposal")?;
        let result = if !approve { Ok("Denied; no file operation performed".into()) }
        else if at.elapsed() >= Duration::from_secs(120) { Err("Proposal expired; review a new proposal".into()) }
        else { self.execute(&action) };
        self.events.push(Event { id, outcome: if !approve { "denied" } else if result.is_ok() { "executed" } else { "failed" }.into() });
        if self.events.len() > 100 { self.events.remove(0); }
        result
    }
    fn execute(&self, action: &Action) -> Result<String, String> {
        let target = self.target(action)?;
        match action {
            Action::Read { .. } => {
                let file = fs::File::open(target).map_err(|e| e.to_string())?;
                let mut bytes = vec![];
                file.take((LIMIT + 1) as u64).read_to_end(&mut bytes).map_err(|e| e.to_string())?;
                if bytes.len() > LIMIT { return Err("Read exceeds 32 KiB".into()); }
                String::from_utf8(bytes).map_err(|_| "Only UTF-8 text is supported".into())
            }
            Action::Create { content, .. } => {
                let mut file = OpenOptions::new().write(true).create_new(true).open(target).map_err(|e| e.to_string())?;
                file.write_all(content.as_bytes()).map_err(|e| e.to_string())?;
                Ok(format!("Created {} bytes", content.len()))
            }
        }
    }
    pub fn events(&self) -> &[Event] { &self.events }
}

pub fn plan_local(prompt: &str, model: &str) -> Result<Action, String> {
    if prompt.len() > 4096 || model.is_empty() || model.len() > 100 { return Err("Invalid prompt or model length".into()); }
    let response = ureq::AgentBuilder::new().timeout(Duration::from_secs(60)).redirects(0).build()
        .post("http://127.0.0.1:11434/api/generate")
        .send_json(serde_json::json!({"model": model, "stream": false, "format": "json", "prompt": prompt,
            "system": "Return only JSON: {\"action\":\"create\",\"path\":\"note.txt\",\"content\":\"text\"} or {\"action\":\"read\",\"path\":\"note.txt\"}. Use flat .txt filenames. No shell commands. Your output is an untrusted proposal requiring human approval."}))
        .map_err(|e| format!("Local Ollama unavailable or request failed: {e}"))?;
    let mut data = String::new();
    response.into_reader().take(131_073).read_to_string(&mut data).map_err(|e| e.to_string())?;
    if data.len() > 131_072 { return Err("Model response too large".into()); }
    let body: serde_json::Value = serde_json::from_str(&data).map_err(|e| e.to_string())?;
    serde_json::from_str(body["response"].as_str().ok_or("Missing model response")?).map_err(|e| format!("Invalid action: {e}"))
}

extern "C" { fn localtrust_gguf_version(data: *const u8, size: usize) -> i32; }
pub fn gguf_version(data: &[u8]) -> Result<u32, String> {
    // C++ reads only within the supplied slice and retains no pointer.
    let v = unsafe { localtrust_gguf_version(data.as_ptr(), data.len()) };
    if v < 0 { Err("Invalid or unsupported GGUF prefix".into()) } else { Ok(v as u32) }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test] fn expired_proposals_are_consumed() {
        let dir = tempfile::tempdir().unwrap(); let mut b = Broker::new(dir.path()).unwrap();
        let p = b.propose(Action::Read { path: "x.txt".into() }).unwrap();
        b.pending.get_mut(&p.id).unwrap().1 = Instant::now() - Duration::from_secs(121);
        assert!(b.decide(p.id, true).unwrap_err().contains("expired"));
        assert!(b.decide(p.id, true).is_err());
    }
}
