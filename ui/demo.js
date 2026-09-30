// Synthetic browser-only adapter. It never reads or writes the host filesystem.
export class Demo {
  constructor() { this.files = new Map(); this.pending = new Map(); this.log = []; this.next = 1; }
  async invoke(command, args = {}) {
    if (command === 'events') return [...this.log];
    if (command === 'plan') throw Error('Ollama requires the native desktop app. Use the synthetic example here.');
    if (command === 'propose') {
      const a = structuredClone(args.action);
      if (!['read', 'create'].includes(a.action) || !/^[a-zA-Z0-9_.-]+\.txt$/.test(a.path) || a.path.includes('..')) throw Error('Use a simple .txt filename.');
      if (a.action === 'create' && (typeof a.content !== 'string' || new TextEncoder().encode(a.content).length > 32768 || this.files.has(a.path))) throw Error('Oversized content or existing filename.');
      const p = { id: this.next++, action: a, expires_in_seconds: 120 };
      this.pending.set(p.id, { action: a, at: Date.now() }); return structuredClone(p);
    }
    if (command === 'decide') {
      const p = this.pending.get(args.id); if (!p) throw Error('Unknown or consumed proposal.');
      this.pending.delete(args.id);
      if (!args.approve) { this.log.push({id: args.id, outcome: 'denied'}); return 'Denied; no operation performed.'; }
      try {
        if (Date.now() - p.at >= 120000) throw Error('Proposal expired.');
        let result;
        if (p.action.action === 'create') {
          if (this.files.has(p.action.path)) throw Error('Overwrite denied.');
          this.files.set(p.action.path, p.action.content); result = 'Created synthetic in-memory file.';
        } else { if (!this.files.has(p.action.path)) throw Error('File not found.'); result = this.files.get(p.action.path); }
        this.log.push({id: args.id, outcome: 'executed'}); return result;
      } catch (e) { this.log.push({id: args.id, outcome: 'failed'}); throw e; }
    }
    throw Error('Unknown command.');
  }
}
