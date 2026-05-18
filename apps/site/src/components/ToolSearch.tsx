import { useEffect, useMemo, useState } from 'react';
import type { Tool } from '../lib/tools';

interface Props {
  tools: Tool[];
  categories: string[];
  languages: string[];
}

function initialParam(name: string) {
  if (typeof window === 'undefined') return null;
  return new URLSearchParams(window.location.search).get(name);
}

function initialPositiveInt(name: string, fallback: number) {
  const value = Number(initialParam(name));
  return Number.isInteger(value) && value > 0 ? value : fallback;
}

export default function ToolSearch({ tools, categories, languages }: Props) {
  const [query, setQuery] = useState(() => initialParam('search') ?? '');
  const [selectedCategory, setSelectedCategory] = useState<string | null>(() => initialParam('category'));
  const [selectedLang, setSelectedLang] = useState<string | null>(() => initialParam('lang'));
  const [selectedRisk, setSelectedRisk] = useState<string | null>(() => initialParam('risk'));
  const [currentPage, setCurrentPage] = useState(() => initialPositiveInt('page', 1));

  const pageSize = 24;

  const risks = ['low', 'medium', 'high', 'critical'];

  const filtered = useMemo(() => {
    return tools.filter((tool) => {
      if (query) {
        const q = query.toLowerCase();
        const match =
          tool.name.toLowerCase().includes(q) ||
          tool.binary.toLowerCase().includes(q) ||
          tool.description.toLowerCase().includes(q) ||
          tool.category.some((c) => c.toLowerCase().includes(q)) ||
          (tool.aliases?.some((a) => a.toLowerCase().includes(q)) ?? false);
        if (!match) return false;
      }
      if (selectedCategory && !tool.category.includes(selectedCategory)) return false;
      if (selectedLang && !tool.lang.includes(selectedLang) && !tool.lang.includes('all')) return false;
      if (selectedRisk && tool.risk !== selectedRisk) return false;
      return true;
    });
  }, [tools, query, selectedCategory, selectedLang, selectedRisk]);

  const activeFilters = [selectedCategory, selectedLang, selectedRisk].filter(Boolean).length;
  const totalPages = Math.max(1, Math.ceil(filtered.length / pageSize));
  const safePage = Math.min(currentPage, totalPages);
  const startIndex = (safePage - 1) * pageSize;
  const visibleTools = filtered.slice(startIndex, startIndex + pageSize);

  useEffect(() => {
    if (currentPage !== safePage) {
      setCurrentPage(safePage);
    }
  }, [currentPage, safePage]);

  useEffect(() => {
    if (typeof window === 'undefined') return;

    const params = new URLSearchParams();
    if (query) params.set('search', query);
    if (selectedCategory) params.set('category', selectedCategory);
    if (selectedLang) params.set('lang', selectedLang);
    if (selectedRisk) params.set('risk', selectedRisk);
    if (safePage > 1) params.set('page', String(safePage));

    const nextUrl = `${window.location.pathname}${params.toString() ? `?${params.toString()}` : ''}${window.location.hash}`;
    window.history.replaceState({}, '', nextUrl);
  }, [query, selectedCategory, selectedLang, selectedRisk, safePage]);

  return (
    <div className="space-y-6">
      <div className="panel">
        <div className="panel-header">
          <span>[QUERY_INTERFACE]</span>
          <span>{String(filtered.length).padStart(3, '0')}_ENTRIES</span>
        </div>

        <div className="space-y-4 p-4 sm:p-6">
          <div className="relative">
            <svg className="absolute left-4 top-1/2 h-5 w-5 -translate-y-1/2 text-acid" fill="none" viewBox="0 0 24 24" stroke="currentColor">
              <path strokeLinecap="square" strokeLinejoin="miter" strokeWidth={2} d="M21 21l-6-6m2-5a7 7 0 11-14 0 7 7 0 0114 0z" />
            </svg>
            <input
              type="text"
              placeholder="SEARCH_RUNBOOK / CATEGORY / COMMAND..."
              value={query}
              onChange={(event) => {
                setQuery(event.target.value);
                setCurrentPage(1);
              }}
              className="terminal-input"
            />
            <span className="pointer-events-none absolute right-4 top-1/2 -translate-y-1/2 text-code text-muted">⌘K</span>
          </div>

          <div className="flex flex-wrap gap-2">
            <FilterGroup
              label="Category"
              options={categories}
              selected={selectedCategory}
              onSelect={(value) => {
                setSelectedCategory(value);
                setCurrentPage(1);
              }}
            />
            <FilterGroup
              label="Language"
              options={languages}
              selected={selectedLang}
              onSelect={(value) => {
                setSelectedLang(value);
                setCurrentPage(1);
              }}
            />
            <FilterGroup
              label="Risk"
              options={risks}
              selected={selectedRisk}
              onSelect={(value) => {
                setSelectedRisk(value);
                setCurrentPage(1);
              }}
            />
            {(query || activeFilters > 0) && (
              <button
                onClick={() => {
                  setQuery('');
                  setSelectedCategory(null);
                  setSelectedLang(null);
                  setSelectedRisk(null);
                  setCurrentPage(1);
                }}
                className="border border-danger/50 bg-danger/10 px-3 py-2 text-code font-bold uppercase tracking-[0.12em] text-danger transition-colors hover:bg-danger hover:text-black"
              >
                [CLEAR]
              </button>
            )}
          </div>
        </div>
      </div>

      <div className="flex items-center justify-between gap-4">
        <p className="text-code uppercase text-muted-variant">
          <span className="text-acid">{filtered.length}</span> entr{filtered.length !== 1 ? 'ies' : 'y'}
          {query || activeFilters ? ' matched' : ' indexed'}
          {filtered.length > 0 && (
            <span className="text-muted"> · showing {startIndex + 1}-{Math.min(startIndex + pageSize, filtered.length)}</span>
          )}
        </p>
        <div className="hidden h-px flex-1 bg-outline-variant sm:block"></div>
        <p className="hidden text-code uppercase text-muted sm:block">[LIVE_DIRECTORY_FEED]</p>
      </div>

      {filtered.length > pageSize && (
        <Pagination currentPage={safePage} totalPages={totalPages} onPageChange={setCurrentPage} />
      )}

      <div className="grid grid-cols-1 gap-[1px] border border-outline-variant bg-outline-variant shadow-hard md:grid-cols-2 xl:grid-cols-3">
        {visibleTools.map((tool) => (
          <ToolCard key={tool.slug} tool={tool} />
        ))}
      </div>

      {filtered.length > pageSize && (
        <Pagination currentPage={safePage} totalPages={totalPages} onPageChange={setCurrentPage} />
      )}

      {filtered.length === 0 && (
        <div className="panel py-16 text-center">
          <p className="text-headline font-bold text-primary">NO SIGNAL</p>
          <p className="mt-2 text-body text-muted-variant">Adjust the query vector or clear filters.</p>
        </div>
      )}
    </div>
  );
}

function Pagination({
  currentPage,
  totalPages,
  onPageChange,
}: {
  currentPage: number;
  totalPages: number;
  onPageChange: (page: number) => void;
}) {
  const pages = pageWindow(currentPage, totalPages);

  return (
    <nav className="panel flex flex-col gap-3 p-3 sm:flex-row sm:items-center sm:justify-between" aria-label="Registry pagination">
      <div className="text-code uppercase text-muted-variant">
        [PAGE <span className="text-acid">{String(currentPage).padStart(2, '0')}</span> / {String(totalPages).padStart(2, '0')}]
      </div>

      <div className="flex flex-wrap items-center gap-2">
        <PageButton disabled={currentPage === 1} onClick={() => onPageChange(1)}>
          [FIRST]
        </PageButton>
        <PageButton disabled={currentPage === 1} onClick={() => onPageChange(currentPage - 1)}>
          ← PREV
        </PageButton>

        <div className="flex flex-wrap gap-1">
          {pages.map((page, index) =>
            page === 'gap' ? (
              <span key={`gap-${index}`} className="border border-outline-variant bg-background px-3 py-2 text-code text-muted">
                …
              </span>
            ) : (
              <button
                key={page}
                type="button"
                aria-current={page === currentPage ? 'page' : undefined}
                onClick={() => onPageChange(page)}
                className={`border px-3 py-2 text-code font-bold transition-colors ${
                  page === currentPage
                    ? 'border-acid bg-acid text-black shadow-hard-acid'
                    : 'border-outline-variant bg-surface-dim text-muted-variant hover:border-acid hover:text-acid'
                }`}
              >
                {String(page).padStart(2, '0')}
              </button>
            ),
          )}
        </div>

        <PageButton disabled={currentPage === totalPages} onClick={() => onPageChange(currentPage + 1)}>
          NEXT →
        </PageButton>
        <PageButton disabled={currentPage === totalPages} onClick={() => onPageChange(totalPages)}>
          [LAST]
        </PageButton>
      </div>
    </nav>
  );
}

function PageButton({ children, disabled, onClick }: { children: React.ReactNode; disabled: boolean; onClick: () => void }) {
  return (
    <button
      type="button"
      disabled={disabled}
      onClick={onClick}
      className="border border-outline-variant bg-surface-dim px-3 py-2 text-code font-bold uppercase tracking-[0.08em] text-muted-variant transition-colors hover:border-acid hover:text-acid disabled:cursor-not-allowed disabled:opacity-35 disabled:hover:border-outline-variant disabled:hover:text-muted-variant"
    >
      {children}
    </button>
  );
}

function pageWindow(currentPage: number, totalPages: number): Array<number | 'gap'> {
  const pages = new Set([1, totalPages, currentPage - 1, currentPage, currentPage + 1]);
  const normalized = [...pages].filter((page) => page >= 1 && page <= totalPages).sort((a, b) => a - b);
  const result: Array<number | 'gap'> = [];

  normalized.forEach((page) => {
    const previous = result[result.length - 1];
    if (typeof previous === 'number' && page - previous > 1) {
      result.push('gap');
    }
    result.push(page);
  });

  return result;
}

function FilterGroup({
  label,
  options,
  selected,
  onSelect,
}: {
  label: string;
  options: string[];
  selected: string | null;
  onSelect: (value: string | null) => void;
}) {
  const [open, setOpen] = useState(false);

  return (
    <div className="relative">
      <button
        onClick={() => setOpen(!open)}
        className={`inline-flex items-center gap-2 border px-3 py-2 text-code font-bold uppercase tracking-[0.12em] transition-colors ${
          selected
            ? 'border-acid bg-acid/10 text-acid shadow-[0_0_12px_rgba(195,244,0,0.12)]'
            : 'border-outline-variant bg-surface-dim text-muted-variant hover:border-acid hover:text-acid'
        }`}
      >
        [{label}]
        {selected && <span className="max-w-[180px] truncate text-primary">{selected}</span>}
        <svg className={`h-3.5 w-3.5 transition-transform ${open ? 'rotate-180' : ''}`} fill="none" viewBox="0 0 24 24" stroke="currentColor">
          <path strokeLinecap="square" strokeLinejoin="miter" strokeWidth={2} d="M19 9l-7 7-7-7" />
        </svg>
      </button>

      {open && (
        <>
          <div className="fixed inset-0 z-10" onClick={() => setOpen(false)} />
          <div className="absolute left-0 top-full z-20 mt-1 max-h-80 min-w-[220px] overflow-y-auto border border-outline bg-surface-dim shadow-hard">
            {selected && (
              <button
                onClick={() => {
                  onSelect(null);
                  setOpen(false);
                }}
                className="w-full border-b border-outline-variant px-3 py-2 text-left text-code uppercase text-danger hover:bg-danger hover:text-black"
              >
                [CLEAR_{label.toUpperCase()}]
              </button>
            )}
            {options.map((opt) => (
              <button
                key={opt}
                onClick={() => {
                  onSelect(opt);
                  setOpen(false);
                }}
                className={`block w-full px-3 py-2 text-left text-code uppercase transition-colors hover:bg-acid hover:text-black ${
                  selected === opt ? 'bg-acid/10 text-acid' : 'text-muted-variant'
                }`}
              >
                {opt}
              </button>
            ))}
          </div>
        </>
      )}
    </div>
  );
}

function ToolCard({ tool }: { tool: Tool }) {
  const riskClass: Record<string, string> = {
    low: 'chip-risk-low',
    medium: 'chip-risk-medium',
    high: 'chip-risk-high',
    critical: 'chip-risk-critical',
  };

  return (
    <a href={`/tools/${tool.slug}`} className="terminal-card group flex min-h-[260px] flex-col p-5">
      <div className="mb-5 flex items-start justify-between gap-4 border-b border-outline-variant pb-4">
        <div className="min-w-0 text-code text-muted-variant">
          ID: <span className="font-bold text-primary">0x{hashTool(tool.slug)}</span>
          <span className="cursor-blink-sm"></span>
        </div>
        <span className={`chip ${riskClass[tool.risk] ?? 'chip-default'}`}>{tool.risk}</span>
      </div>

      <h3 className="mb-3 truncate text-title font-bold text-primary transition-colors group-hover:text-acid">{tool.binary}</h3>
      <p className="mb-6 line-clamp-3 min-h-[64px] text-body text-muted-variant">{tool.description}</p>

      <div className="mt-auto grid grid-cols-2 gap-3 border border-outline-variant bg-background p-3 text-code">
        <div>
          <div className="mb-1 text-[10px] uppercase tracking-[0.14em] text-muted">[CATEGORY]</div>
          <div className="truncate font-bold text-primary">{tool.category[0] ?? 'uncategorized'}</div>
        </div>
        <div>
          <div className="mb-1 text-[10px] uppercase tracking-[0.14em] text-muted">[LANG]</div>
          <div className="truncate font-bold text-primary">{tool.lang.includes('all') ? 'all' : tool.lang.slice(0, 2).join('/')}</div>
        </div>
      </div>

      <div className="mt-4 flex flex-wrap gap-1.5">
        {tool.category.slice(0, 2).map((cat) => (
          <span key={cat} className="chip chip-default">{cat}</span>
        ))}
        {tool.effects?.slice(0, 2).map((effect) => (
          <span key={effect} className="chip chip-default">{effect}</span>
        ))}
      </div>
    </a>
  );
}

function hashTool(value: string) {
  let hash = 0;
  for (let index = 0; index < value.length; index += 1) {
    hash = (hash * 31 + value.charCodeAt(index)) >>> 0;
  }
  return hash.toString(16).toUpperCase().slice(0, 5).padStart(5, '0');
}
