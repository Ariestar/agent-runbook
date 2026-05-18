import fs from 'node:fs';
import path from 'node:path';
import yaml from 'js-yaml';

export interface Tool {
  slug: string;
  name: string;
  binary: string;
  description: string;
  category: string[];
  lang: string[];
  platform?: string[];
  risk: string;
  effects: string[];
  aliases?: string[];
  use_when?: string[];
  avoid_when?: string[];
  guardrails?: string[];
  docs?: string;
}

export function loadTools(): Tool[] {
  const toolsDir = path.resolve('../../data/tools');
  const files = fs.readdirSync(toolsDir).filter((f) => f.endsWith('.yaml'));

  return files
    .map((file) => {
      const content = fs.readFileSync(path.join(toolsDir, file), 'utf-8');
      const data = yaml.load(content) as Record<string, unknown>;

      const riskRaw = data.risk as string | { level: string; effects?: string[] };
      const risk = typeof riskRaw === 'string' ? riskRaw : riskRaw?.level ?? 'medium';
      const effects =
        typeof riskRaw === 'object' && riskRaw?.effects
          ? riskRaw.effects
          : ((data.effects as string[]) ?? []);

      const langRaw = data.lang;
      const lang = Array.isArray(langRaw) ? langRaw : typeof langRaw === 'string' ? [langRaw] : ['all'];

      const description = (data.summary as string) ?? (data.description as string) ?? '';

      return {
        slug: file.replace('.yaml', ''),
        name: (data.name as string) ?? file.replace('.yaml', ''),
        binary: (data.binary as string) ?? (data.name as string) ?? file.replace('.yaml', ''),
        description,
        category: (data.category as string[]) ?? [],
        lang,
        platform: data.platform as string[] | undefined,
        risk,
        effects,
        aliases: data.aliases as string[] | undefined,
        use_when: data.use_when as string[] | undefined,
        avoid_when: data.avoid_when as string[] | undefined,
        guardrails: data.guardrails as string[] | undefined,
        docs: (data.docs as string) ?? (data.homepage as string) ?? undefined,
      } as Tool;
    })
    .sort((a, b) => a.name.localeCompare(b.name));
}

export function getCategories(tools: Tool[]): string[] {
  const set = new Set<string>();
  for (const tool of tools) {
    for (const cat of tool.category) {
      set.add(cat);
    }
  }
  return [...set].sort();
}

export function getLanguages(tools: Tool[]): string[] {
  const set = new Set<string>();
  for (const tool of tools) {
    for (const l of tool.lang) {
      set.add(l);
    }
  }
  return [...set].sort();
}

export function getRisks(): string[] {
  return ['low', 'medium', 'high', 'critical'];
}
