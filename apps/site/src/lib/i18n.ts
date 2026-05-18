export const defaultLocale = 'en';
export const locales = ['en', 'zh-CN'] as const;
export type Locale = (typeof locales)[number];

export const localeNames: Record<Locale, string> = {
  en: 'English',
  'zh-CN': '中文',
};

export const localeShortNames: Record<Locale, string> = {
  en: 'EN',
  'zh-CN': '中',
};

const messages: Record<Locale, Record<string, string>> = {
  en: {
    'layout.description': 'The operating contract layer for AI coding agents: local tool discovery, command selection, risk metadata, and guardrails.',

    'header.nav.registry': 'REGISTRY',
    'header.nav.categories': 'CATEGORIES',
    'header.nav.protocol': 'PROTOCOL',
    'header.nav.contribute': 'CONTRIBUTE',
    'header.search.placeholder': 'SEARCH RUNBOOK_REGISTRY...',
    'header.star': 'STAR_ON_GITHUB',
    'header.language': 'Language',
    'theme.label': 'Theme',

    'hero.status': '[RUNBOOK] OPERATING_CONTRACT_ONLINE',
    'hero.title.brand': 'RUNBOOK',
    'hero.title.beforeShell': 'BEFORE THE SHELL.',
    'hero.title.consequences': 'COMMANDS WITH CONSEQUENCES.',
    'hero.body': 'The operating contract layer for AI coding agents: scan the local environment, read repository preferences, query the command registry, and turn every shell action into an auditable decision.',
    'hero.cta.registry': '[ OPEN RUNBOOK REGISTRY ]',
    'hero.cta.protocol': '[ READ OPERATING PROTOCOL ]',
    'hero.trace': '[RUNBOOK_DECISION_TRACE]',
    'hero.confidence': '[AGENT_CONFIDENCE]',
    'hero.terminal.0': 'runbook scan --minimal',
    'hero.terminal.1': 'detected pnpm / astro / typescript',
    'hero.terminal.2': 'runbook category test lint --lang typescript',
    'hero.terminal.3': 'preferred: pnpm scripts · risk: medium',
    'hero.terminal.4': 'runbook prefer',
    'hero.terminal.5': 'guardrail: do not mix package managers',
    'hero.terminal.6': 'pnpm build',
    'hero.terminal.7': '200 routes generated · ship confidence',

    'stat.tools': 'TOOLS',
    'stat.categories': 'CATEGORIES',
    'stat.languages': 'LANGUAGES',

    'home.featured': '[FEATURED_RUNBOOK_ENTRIES]',
    'home.viewCategories': 'VIEW_CATEGORIES →',
    'home.footer.core': '[© 2026 OPERATING_CONTRACT_CORE]',

    'tools.title': 'RUNBOOK_REGISTRY',
    'tools.metaTitle': 'Registry — Runbook',
    'tools.label': '[FULL_RUNBOOK_REGISTRY]',
    'tools.body': 'Search every indexed command by binary, category, language, effect, and risk profile. Built for agents that need to choose the right executable before touching the shell.',

    'categories.metaTitle': 'Categories — Runbook',
    'categories.label': '[DIRECTORY_SEGMENTATION]',
    'categories.title': 'CATEGORIES',
    'categories.body': 'Browse the Runbook registry by operational intent. Each category is a tool-selection vector for agents choosing the safest executable path.',
    'categories.tools': 'tools',

    'agents.metaTitle': 'For Agents — Runbook',
    'agents.label': '[RUNBOOK_PROTOCOL]',
    'agents.title': 'OPERATING_PROTOCOL',
    'agents.body': 'A compact operating contract for choosing commands by task, language, availability, repository preference, and risk. Context before execution.',
    'agents.principles': '[RUNBOOK_SELECTION_PRINCIPLES]',
    'agents.rules': '05_RULES',
    'agents.rule.1.title': 'Start with the task category',
    'agents.rule.1.body': 'Need to search code? Query search tools. Need tests? Query test tools.',
    'agents.rule.2.title': 'Filter by language',
    'agents.rule.2.body': 'Prefer tools with the project ecosystem or lang: all.',
    'agents.rule.3.title': 'Prefer lower risk',
    'agents.rule.3.body': 'If a low-risk read-only tool can do the job, do not escalate.',
    'agents.rule.4.title': 'Check local evidence',
    'agents.rule.4.body': 'Runbook scan tells the agent what exists locally and what the project requires.',
    'agents.rule.5.title': 'Respect guardrails',
    'agents.rule.5.body': 'High and critical risk require explicit confirmation paths.',
    'agents.effects': '[EFFECTS_MATRIX]',
    'agents.risk': 'RISK',
    'agents.safe': 'safe',
    'agents.caution': 'caution',
    'agents.danger': 'danger',
    'agents.effect.read_files': 'Reads only; no side effects.',
    'agents.effect.write_files': 'Modifies local files.',
    'agents.effect.execute_code': 'Runs arbitrary local commands.',
    'agents.effect.network_access': 'Makes network calls.',
    'agents.effect.secret_exposure': 'May reveal secrets.',
    'agents.interface': '[RUNBOOK_INTERFACE]',
    'agents.cli': 'CLI',
    'agents.interface.body': 'Use Runbook to combine the registry with local facts and repository preferences.',
    'agents.flow': '[DECISION_FLOW]',
    'agents.closedLoop': 'CLOSED_LOOP',
    'agents.step.category': 'category',
    'agents.step.language': 'language',
    'agents.step.availability': 'availability',
    'agents.step.risk': 'risk',
    'agents.step.guardrails': 'guardrails',
    'agents.step.confirmation': 'confirmation',

    'contribute.metaTitle': 'Contribute — Runbook',
    'contribute.label': '[RUNBOOK_WRITE_PROTOCOL]',
    'contribute.title': 'CONTRIBUTE_A_RUNBOOK_ENTRY',
    'contribute.body': 'Add executable CLI metadata that agents can trust: decision guidance, guardrails, effects, and risk semantics.',
    'contribute.acceptance': '[ACCEPTANCE_CRITERIA]',
    'contribute.goodFit': 'GOOD_FIT',
    'contribute.rejection': '[REJECTION_CRITERIA]',
    'contribute.notFit': 'NOT_A_FIT',
    'contribute.good.1': 'Agent can execute it directly as a CLI command',
    'contribute.good.2': 'Helps search, read, edit, test, debug, or deploy software',
    'contribute.good.3': 'Has clear risk and guardrail semantics',
    'contribute.good.4': 'Useful for AI coding workflows',
    'contribute.bad.1': 'Libraries or frameworks with no CLI surface',
    'contribute.bad.2': 'GUI-only applications',
    'contribute.bad.3': 'General technology encyclopedias',
    'contribute.bad.4': 'Tools with no clear agent use case',
    'contribute.sequence': '[SUBMISSION_SEQUENCE]',
    'contribute.steps': '04_STEPS',
    'contribute.step.1': 'Fork the repository on GitHub',
    'contribute.step.2': 'Create data/tools/your-tool.yaml',
    'contribute.step.3': 'Fill required metadata fields',
    'contribute.step.4': 'Open a pull request',
    'contribute.template': '[YAML_TEMPLATE]',
    'contribute.spec': 'SPEC',

    'search.panel': '[QUERY_INTERFACE]',
    'search.entries': 'ENTRIES',
    'search.placeholder': 'SEARCH_RUNBOOK / CATEGORY / COMMAND...',
    'search.category': 'Category',
    'search.language': 'Language',
    'search.risk': 'Risk',
    'search.clear': '[CLEAR]',
    'search.clearFilter': 'CLEAR',
    'search.entry': 'entry',
    'search.entriesLower': 'entries',
    'search.matched': 'matched',
    'search.indexed': 'indexed',
    'search.showing': 'showing',
    'search.liveFeed': '[LIVE_DIRECTORY_FEED]',
    'search.noSignal': 'NO SIGNAL',
    'search.noSignal.body': 'Adjust the query vector or clear filters.',
    'search.pagination': 'Registry pagination',
    'search.page': 'PAGE',
    'search.first': '[FIRST]',
    'search.prev': '← PREV',
    'search.next': 'NEXT →',
    'search.last': '[LAST]',

    'tool.id': 'ID',
    'tool.category': '[CATEGORY]',
    'tool.lang': '[LANG]',
    'tool.back': '← [BACK_TO_DIRECTORY]',
    'tool.profile': '[TOOL_PROFILE]',
    'tool.riskSuffix': 'risk',
    'tool.langPrefix': 'lang',
    'tool.platformPrefix': 'platform',
    'tool.useWhen': '[USE_WHEN]',
    'tool.allow': 'ALLOW',
    'tool.avoidWhen': '[AVOID_WHEN]',
    'tool.block': 'BLOCK',
    'tool.guardrails': '[GUARDRAILS]',
    'tool.mandatory': 'MANDATORY',
    'tool.details': '[DETAILS]',
    'tool.rawMeta': 'RAW_META',
    'tool.binary': 'Binary',
    'tool.aliases': 'Aliases',
    'tool.effects': 'Effects',
    'tool.documentation': 'Documentation',

    'risk.low': 'low',
    'risk.medium': 'medium',
    'risk.high': 'high',
    'risk.critical': 'critical',
  },
  'zh-CN': {
    'layout.description': 'Runbook 帮 AI 编程代理先看清环境、选对命令、理解风险，再动手执行。',

    'header.nav.registry': '工具库',
    'header.nav.categories': '分类',
    'header.nav.protocol': '给 Agent',
    'header.nav.contribute': '参与贡献',
    'header.search.placeholder': '搜索工具、分类或命令...',
    'header.star': 'GitHub Star',
    'header.language': '语言',
    'theme.label': '主题',

    'hero.status': '[RUNBOOK] READY FOR AGENTS',
    'hero.title.brand': 'RUNBOOK',
    'hero.title.beforeShell': '让 Agent 在执行前',
    'hero.title.consequences': '先选对命令。',
    'hero.body': 'Runbook 是给 AI 编程代理用的命令决策层：先扫描本地环境，读取仓库偏好，再根据任务、语言、风险和副作用选择合适的 CLI。每一次 shell 操作，都有依据可查。',
    'hero.cta.registry': '[ 查看工具库 ]',
    'hero.cta.protocol': '[ 了解 Agent 使用方式 ]',
    'hero.trace': '[决策过程]',
    'hero.confidence': '[执行把握]',
    'hero.terminal.0': 'runbook scan --minimal',
    'hero.terminal.1': '发现 pnpm / astro / typescript',
    'hero.terminal.2': 'runbook category test lint --lang typescript',
    'hero.terminal.3': '推荐：pnpm scripts · 风险：中',
    'hero.terminal.4': 'runbook prefer',
    'hero.terminal.5': '规则：不要混用包管理器',
    'hero.terminal.6': 'pnpm build',
    'hero.terminal.7': '200 个页面已生成，可以发布',

    'stat.tools': '工具',
    'stat.categories': '分类',
    'stat.languages': '语言',

    'home.featured': '[精选工具]',
    'home.viewCategories': '查看分类 →',
    'home.footer.core': '[© 2026 Runbook]',

    'tools.title': '工具库',
    'tools.metaTitle': '工具库 — Runbook',
    'tools.label': '[所有工具]',
    'tools.body': '按命令名、分类、语言、副作用和风险等级搜索已收录的 CLI。适合需要在执行前先判断“该用哪个工具”的 Agent。',

    'categories.metaTitle': '分类 — Runbook',
    'categories.label': '[按任务查找]',
    'categories.title': '分类',
    'categories.body': '按任务意图浏览 Runbook 工具库。搜索、读取、编辑、测试、部署——每个分类都对应 Agent 的一个常见动作。',
    'categories.tools': '个工具',

    'agents.metaTitle': '给 Agent — Runbook',
    'agents.label': '[Agent 使用指南]',
    'agents.title': '先判断，再执行',
    'agents.body': 'Runbook 用任务、语言、可用性、仓库偏好和风险信息，帮 Agent 在执行命令前做一次清晰判断。',
    'agents.principles': '[选择工具的基本原则]',
    'agents.rules': '5 条规则',
    'agents.rule.1.title': '先看任务类型',
    'agents.rule.1.body': '要搜代码，就看搜索类工具；要跑测试，就看测试类工具。',
    'agents.rule.2.title': '匹配项目语言',
    'agents.rule.2.body': '优先使用项目生态里的工具；通用工具则看 lang: all。',
    'agents.rule.3.title': '能低风险就低风险',
    'agents.rule.3.body': '只读工具能完成的事，就不要升级到会写文件或执行代码的工具。',
    'agents.rule.4.title': '先确认本地情况',
    'agents.rule.4.body': 'runbook scan 会告诉 Agent 本机有哪些工具、项目需要什么。',
    'agents.rule.5.title': '遵守安全边界',
    'agents.rule.5.body': '高风险和关键风险操作，需要走明确的确认流程。',
    'agents.effects': '[副作用说明]',
    'agents.risk': '风险',
    'agents.safe': '安全',
    'agents.caution': '谨慎',
    'agents.danger': '危险',
    'agents.effect.read_files': '只读取文件，不改变状态。',
    'agents.effect.write_files': '会修改本地文件。',
    'agents.effect.execute_code': '会在本地执行命令。',
    'agents.effect.network_access': '会访问网络。',
    'agents.effect.secret_exposure': '可能接触或暴露密钥。',
    'agents.interface': '[Runbook 接口]',
    'agents.cli': 'CLI',
    'agents.interface.body': '用 Runbook 把工具库、本地环境和仓库偏好合在一起，形成可执行前的判断依据。',
    'agents.flow': '[决策流程]',
    'agents.closedLoop': '闭环',
    'agents.step.category': '任务',
    'agents.step.language': '语言',
    'agents.step.availability': '可用性',
    'agents.step.risk': '风险',
    'agents.step.guardrails': '规则',
    'agents.step.confirmation': '确认',

    'contribute.metaTitle': '参与贡献 — Runbook',
    'contribute.label': '[如何添加工具]',
    'contribute.title': '贡献一个 Runbook 条目',
    'contribute.body': '为 CLI 工具补充 Agent 能理解的元数据：什么时候用、什么时候不用、有什么风险、需要哪些安全边界。',
    'contribute.acceptance': '[适合收录]',
    'contribute.goodFit': '适合',
    'contribute.rejection': '[不适合收录]',
    'contribute.notFit': '不适合',
    'contribute.good.1': 'Agent 可以直接通过命令行调用',
    'contribute.good.2': '能帮助完成搜索、读取、编辑、测试、调试或部署',
    'contribute.good.3': '风险和使用边界可以说清楚',
    'contribute.good.4': '对 AI 编程工作流有实际帮助',
    'contribute.bad.1': '没有命令行入口的库或框架',
    'contribute.bad.2': '只能用图形界面操作的应用',
    'contribute.bad.3': '泛泛介绍技术概念的百科内容',
    'contribute.bad.4': '看不出 Agent 具体会怎么使用的工具',
    'contribute.sequence': '[提交流程]',
    'contribute.steps': '4 步',
    'contribute.step.1': 'Fork GitHub 仓库',
    'contribute.step.2': '创建 data/tools/your-tool.yaml',
    'contribute.step.3': '填写必要的元数据字段',
    'contribute.step.4': '提交 Pull Request',
    'contribute.template': '[YAML 示例]',
    'contribute.spec': '字段说明',

    'search.panel': '[搜索]',
    'search.entries': '条目',
    'search.placeholder': '搜索工具、分类或命令...',
    'search.category': '分类',
    'search.language': '语言',
    'search.risk': '风险',
    'search.clear': '[清除]',
    'search.clearFilter': '清除',
    'search.entry': '条',
    'search.entriesLower': '条',
    'search.matched': '匹配',
    'search.indexed': '已收录',
    'search.showing': '显示',
    'search.liveFeed': '[工具列表]',
    'search.noSignal': '没有找到结果',
    'search.noSignal.body': '换个关键词，或清除筛选条件后再试。',
    'search.pagination': '工具库分页',
    'search.page': '第',
    'search.first': '[首页]',
    'search.prev': '← 上一页',
    'search.next': '下一页 →',
    'search.last': '[末页]',

    'tool.id': 'ID',
    'tool.category': '[分类]',
    'tool.lang': '[语言]',
    'tool.back': '← [返回工具库]',
    'tool.profile': '[工具信息]',
    'tool.riskSuffix': '风险',
    'tool.langPrefix': '语言',
    'tool.platformPrefix': '平台',
    'tool.useWhen': '[什么时候用]',
    'tool.allow': '适用',
    'tool.avoidWhen': '[什么时候不用]',
    'tool.block': '避免',
    'tool.guardrails': '[使用边界]',
    'tool.mandatory': '注意',
    'tool.details': '[详情]',
    'tool.rawMeta': '原始元数据',
    'tool.binary': '命令',
    'tool.aliases': '别名',
    'tool.effects': '副作用',
    'tool.documentation': '文档',

    'risk.low': '低',
    'risk.medium': '中',
    'risk.high': '高',
    'risk.critical': '关键',
  },
};

export function isSupportedLocale(locale: string | undefined): locale is Locale {
  return Boolean(locale && (locales as readonly string[]).includes(locale));
}

export function normalizeLocale(locale: string | undefined | null): Locale {
  return isSupportedLocale(locale ?? undefined) ? locale : defaultLocale;
}

export function getNonDefaultLocales(): Locale[] {
  return locales.filter((locale) => locale !== defaultLocale);
}

export function getLocaleFromUrl(url: URL): Locale {
  const firstSegment = url.pathname.split('/').filter(Boolean)[0];
  return normalizeLocale(firstSegment);
}

export function stripLocale(pathname: string): string {
  const parts = pathname.split('/').filter(Boolean);
  if (isSupportedLocale(parts[0])) {
    parts.shift();
  }
  return `/${parts.join('/')}`.replace(/\/$/, '') || '/';
}

export function localePath(locale: Locale, path = '/'): string {
  const normalizedPath = path.startsWith('/') ? path : `/${path}`;
  const stripped = stripLocale(normalizedPath);
  if (locale === defaultLocale) {
    return stripped;
  }
  return stripped === '/' ? `/${locale}` : `/${locale}${stripped}`;
}

export function switchLocalePath(pathname: string, locale: Locale): string {
  return localePath(locale, stripLocale(pathname));
}

export function t(locale: Locale | string | undefined, key: string): string {
  const normalized = normalizeLocale(locale);
  return messages[normalized][key] ?? messages[defaultLocale][key] ?? key;
}
