const state = {
  tools: [],
  query: "",
  category: "",
  lang: "",
  platform: "",
  risk: "",
};

const els = {
  stats: document.querySelector("#stats"),
  count: document.querySelector("#count"),
  tools: document.querySelector("#tools"),
  search: document.querySelector("#search"),
  category: document.querySelector("#category"),
  lang: document.querySelector("#lang"),
  platform: document.querySelector("#platform"),
  risk: document.querySelector("#risk"),
  template: document.querySelector("#tool-card"),
};

const response = await fetch("./tools.json");
state.tools = await response.json();

initFilters();
render();

els.search.addEventListener("input", () => {
  state.query = els.search.value.trim().toLowerCase();
  render();
});

els.category.addEventListener("change", () => {
  state.category = els.category.value;
  render();
});

els.lang.addEventListener("change", () => {
  state.lang = els.lang.value;
  render();
});

els.platform.addEventListener("change", () => {
  state.platform = els.platform.value;
  render();
});

els.risk.addEventListener("change", () => {
  state.risk = els.risk.value;
  render();
});

function initFilters() {
  const categories = [
    ...new Set(state.tools.flatMap((tool) => tool.category || [])),
  ].sort();
  for (const category of categories) {
    const option = document.createElement("option");
    option.value = category;
    option.textContent = category;
    els.category.append(option);
  }

  const languages = [...new Set(state.tools.flatMap((tool) => tool.lang || []))].sort();
  for (const lang of languages) {
    const option = document.createElement("option");
    option.value = lang;
    option.textContent = lang;
    els.lang.append(option);
  }

  const platforms = [...new Set(state.tools.flatMap((tool) => tool.platform || []))].sort();
  for (const platform of platforms) {
    const option = document.createElement("option");
    option.value = platform;
    option.textContent = platform;
    els.platform.append(option);
  }

  els.stats.textContent = `${state.tools.length} tools · YAML source · JSON runtime index`;
}

function render() {
  const tools = filteredTools();
  els.count.textContent = `${tools.length} shown`;
  els.tools.replaceChildren(...tools.map(renderCard));
}

function filteredTools() {
  return state.tools.filter((tool) => {
    const haystack = [
      tool.name,
      tool.binary,
      ...(tool.category || []),
      ...(tool.lang || []),
      ...(tool.platform || []),
      tool.summary,
      ...(tool.use_when || []),
      ...(tool.avoid_when || []),
      ...(tool.guardrails || []),
      ...(tool.risk?.effects || []),
    ]
      .join(" ")
      .toLowerCase();

    return (
      (!state.query || haystack.includes(state.query)) &&
      (!state.category || (tool.category || []).includes(state.category)) &&
      (!state.lang || (tool.lang || []).includes(state.lang)) &&
      (!state.platform || (tool.platform || []).includes(state.platform)) &&
      (!state.risk || tool.risk?.level === state.risk)
    );
  });
}

function renderCard(tool) {
  const card = els.template.content.firstElementChild.cloneNode(true);
  card.querySelector("h3").textContent = tool.name;
  card.querySelector(".binary").textContent = `$ ${tool.binary}`;
  card.querySelector(".summary").textContent = tool.summary;
  const platform = (tool.platform || []).length ? ` · ${(tool.platform || []).join(", ")}` : "";
  card.querySelector(".meta").textContent = `${(tool.category || []).join(", ")} · ${(tool.lang || []).join(", ")}${platform}`;

  const risk = card.querySelector(".risk");
  risk.textContent = tool.risk.level;
  risk.classList.add(tool.risk.level);

  fillList(card.querySelector(".use"), tool.use_when);
  fillList(card.querySelector(".avoid"), tool.avoid_when);
  fillList(card.querySelector(".guardrails"), tool.guardrails);

  const effects = card.querySelector(".effects");
  effects.replaceChildren(
    ...tool.risk.effects.map((effect) => {
      const chip = document.createElement("span");
      chip.className = "chip";
      chip.textContent = effect;
      return chip;
    }),
  );

  card.querySelector(".homepage").href = tool.homepage;
  card.querySelector(".docs").href = tool.docs;
  return card;
}

function fillList(list, items) {
  list.replaceChildren(
    ...(items || []).map((item) => {
      const li = document.createElement("li");
      li.textContent = item;
      return li;
    }),
  );
}
