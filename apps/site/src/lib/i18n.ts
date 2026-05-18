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
    'hero.cta.protocol': '[ GITHUB ]',
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
    'contribute.addTool': 'Add New Tool',
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
    'tool.similar': '[SIMILAR_TOOLS]',

    'risk.low': 'low',
    'risk.medium': 'medium',
    'risk.high': 'high',
    'risk.critical': 'critical',
  },
  'zh-CN': {
    'layout.description': 'Runbook —— AI 编程 Agent 的工具选择引擎。扫描本地环境，识别可用工具，评估风险，辅助每次命令行决策。',

    'header.nav.registry': '工具库',
    'header.nav.categories': '分类',
    'header.nav.protocol': '协议',
    'header.nav.contribute': '贡献',
    'header.search.placeholder': '搜索工具或命令...',
    'header.star': 'GitHub Star',
    'header.language': '语言',
    'theme.label': '主题',

    'hero.status': '[RUNBOOK] 已就绪',
    'hero.title.brand': 'RUNBOOK',
    'hero.title.beforeShell': '先想清楚，',
    'hero.title.consequences': '再执行。',
    'hero.body': 'AI 编程 Agent 的工具选择引擎。扫描本地环境，读取仓库配置，按任务类型和风险等级匹配最合适的 CLI 工具。每次执行均可审计。',
    'hero.cta.registry': '[ 浏览工具库 ]',
    'hero.cta.protocol': '[ GitHub ]',
    'hero.trace': '[决策记录]',
    'hero.confidence': '[置信度]',
    'hero.terminal.0': 'runbook scan --minimal',
    'hero.terminal.1': '检测到 pnpm / astro / typescript',
    'hero.terminal.2': 'runbook category test lint --lang typescript',
    'hero.terminal.3': '推荐 pnpm scripts · 风险：中',
    'hero.terminal.4': 'runbook prefer',
    'hero.terminal.5': '约束：禁止混用包管理器',
    'hero.terminal.6': 'pnpm build',
    'hero.terminal.7': '生成 200 页 · 可以部署',

    'stat.tools': '工具',
    'stat.categories': '分类',
    'stat.languages': '语言',

    'home.featured': '[精选工具]',
    'home.viewCategories': '查看分类 →',
    'home.footer.core': '[© 2026 Runbook]',

    'tools.title': '工具库',
    'tools.metaTitle': '工具库 — Runbook',
    'tools.label': '[全部工具]',
    'tools.body': '按名称、分类、语言、影响范围和风险等级检索已收录的 CLI 工具。帮助 Agent 在执行前选择最合适的命令。',

    'categories.metaTitle': '分类 — Runbook',
    'categories.label': '[按用途浏览]',
    'categories.title': '分类',
    'categories.body': '按任务意图浏览工具库。搜索、读取、编辑、测试、部署——每个分类对应 Agent 的一类常见操作。',
    'categories.tools': '个工具',

    'agents.metaTitle': '协议 — Runbook',
    'agents.label': '[Agent 协议]',
    'agents.title': '操作协议',
    'agents.body': '基于任务类型、项目语言、本地可用性和风险等级，为 Agent 提供结构化的命令选择依据。',
    'agents.principles': '[选择原则]',
    'agents.rules': '5 条规则',
    'agents.rule.1.title': '从任务类型出发',
    'agents.rule.1.body': '需要搜索代码？查询搜索类工具。需要跑测试？查询测试类工具。',
    'agents.rule.2.title': '匹配项目语言',
    'agents.rule.2.body': '优先使用项目生态内的工具，通用工具看 lang: all。',
    'agents.rule.3.title': '优先低风险',
    'agents.rule.3.body': '只读工具能完成的任务，不应升级到写文件或执行代码的工具。',
    'agents.rule.4.title': '检查本地环境',
    'agents.rule.4.body': 'runbook scan 会告知 Agent 本机可用的工具和项目要求。',
    'agents.rule.5.title': '遵守安全约束',
    'agents.rule.5.body': '高风险和关键风险操作必须经过明确的确认流程。',
    'agents.effects': '[影响矩阵]',
    'agents.risk': '风险',
    'agents.safe': '安全',
    'agents.caution': '谨慎',
    'agents.danger': '危险',
    'agents.effect.read_files': '仅读取，无副作用。',
    'agents.effect.write_files': '修改本地文件。',
    'agents.effect.execute_code': '执行本地命令。',
    'agents.effect.network_access': '发起网络请求。',
    'agents.effect.secret_exposure': '可能暴露密钥或凭证。',
    'agents.interface': '[Runbook 接口]',
    'agents.cli': 'CLI',
    'agents.interface.body': '通过 Runbook CLI 将工具库、本地环境和仓库配置结合，输出结构化的工具选择建议。',
    'agents.flow': '[决策流程]',
    'agents.closedLoop': '闭环',
    'agents.step.category': '任务',
    'agents.step.language': '语言',
    'agents.step.availability': '可用性',
    'agents.step.risk': '风险',
    'agents.step.guardrails': '约束',
    'agents.step.confirmation': '确认',

    'contribute.metaTitle': '贡献 — Runbook',
    'contribute.label': '[提交工具]',
    'contribute.title': '贡献新工具',
    'contribute.body': '为 CLI 工具添加 Agent 可读的元数据：适用场景、风险等级、安全约束和决策指导。',
    'contribute.acceptance': '[收录标准]',
    'contribute.goodFit': '适合收录',
    'contribute.rejection': '[不予收录]',
    'contribute.notFit': '不适合',
    'contribute.good.1': 'Agent 可直接通过命令行调用',
    'contribute.good.2': '用于搜索、读写、测试、调试或部署',
    'contribute.good.3': '风险和约束语义明确',
    'contribute.good.4': '对 AI 编程工作流有实际价值',
    'contribute.bad.1': '无 CLI 接口的库或框架',
    'contribute.bad.2': '纯 GUI 应用',
    'contribute.bad.3': '泛技术百科内容',
    'contribute.bad.4': '无明确 Agent 使用场景的工具',
    'contribute.sequence': '[提交流程]',
    'contribute.steps': '4 步',
    'contribute.step.1': 'Fork 仓库',
    'contribute.step.2': '创建 data/tools/your-tool.yaml',
    'contribute.step.3': '填写必要的元数据字段',
    'contribute.step.4': '提交 Pull Request',
    'contribute.addTool': '添加新工具',
    'contribute.template': '[YAML 模板]',
    'contribute.spec': '字段说明',

    'search.panel': '[搜索]',
    'search.entries': '条',
    'search.placeholder': '搜索工具或命令...',
    'search.category': '分类',
    'search.language': '语言',
    'search.risk': '风险',
    'search.clear': '[清除]',
    'search.clearFilter': '清除',
    'search.entry': '条',
    'search.entriesLower': '条',
    'search.matched': '匹配',
    'search.indexed': '已收录',
    'search.showing': '当前',
    'search.liveFeed': '[结果]',
    'search.noSignal': '未找到相关结果',
    'search.noSignal.body': '请调整搜索关键词或清除筛选条件。',
    'search.pagination': '分页',
    'search.page': '第',
    'search.first': '[首页]',
    'search.prev': '← 上一页',
    'search.next': '下一页 →',
    'search.last': '[末页]',

    'tool.id': 'ID',
    'tool.category': '[分类]',
    'tool.lang': '[语言]',
    'tool.back': '← [返回工具库]',
    'tool.profile': '[工具详情]',
    'tool.riskSuffix': '风险',
    'tool.langPrefix': '语言',
    'tool.platformPrefix': '平台',
    'tool.useWhen': '[适用场景]',
    'tool.allow': '推荐',
    'tool.avoidWhen': '[不适用场景]',
    'tool.block': '避免',
    'tool.guardrails': '[安全约束]',
    'tool.mandatory': '必须遵守',
    'tool.details': '[详细信息]',
    'tool.rawMeta': '原始元数据',
    'tool.binary': '命令',
    'tool.aliases': '别名',
    'tool.effects': '影响',
    'tool.documentation': '文档',
    'tool.similar': '[类似工具]',

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
