import init, { WasmSimulation } from './pkg/evolution_sim.js';

// ===== State =====
let sim = null;
let playing = false;
let speed = 1;
let terrainImageData = null;
let animFrameId = null;

// Canvas
const canvas = document.getElementById('world-canvas');
const ctx = canvas.getContext('2d');

// Graph
const graphCanvas = document.getElementById('graph-canvas');
const graphCtx = graphCanvas.getContext('2d');

// Scale factor for rendering
const SCALE = 4;

// ===== DOM References =====
const seedInput = document.getElementById('seed-input');
const btnNew = document.getElementById('btn-new');
const btnPlay = document.getElementById('btn-play');
const btnStep = document.getElementById('btn-step');
const speedSlider = document.getElementById('speed-slider');
const speedLabel = document.getElementById('speed-label');
const tickDisplay = document.getElementById('tick-display');
const popDisplay = document.getElementById('pop-display');
const speciesDisplay = document.getElementById('species-display');
const viewMode = document.getElementById('view-mode');
const showCreatures = document.getElementById('show-creatures');
const loadingOverlay = document.getElementById('loading-overlay');
const speciesList = document.getElementById('species-list');
const tileInfo = document.getElementById('tile-info');
const tabButtons = document.querySelectorAll('.tab-btn');

// ===== Initialize =====
async function main() {
    await init();

    setupEventListeners();
    createSimulation(parseInt(seedInput.value) || 42);
}

function createSimulation(seed) {
    loadingOverlay.classList.remove('hidden');

    // Use requestAnimationFrame to let the loading overlay render
    requestAnimationFrame(() => {
        requestAnimationFrame(() => {
            sim = new WasmSimulation(seed);
            terrainImageData = null;
            renderTerrain();
            render();
            loadingOverlay.classList.add('hidden');
            updateStats();
        });
    });
}

// ===== Event Listeners =====
function setupEventListeners() {
    btnNew.addEventListener('click', () => {
        playing = false;
        btnPlay.textContent = '\u25B6 Play';
        createSimulation(parseInt(seedInput.value) || 42);
    });

    btnPlay.addEventListener('click', () => {
        playing = !playing;
        btnPlay.textContent = playing ? '\u23F8 Pause' : '\u25B6 Play';
        if (playing) gameLoop();
    });

    btnStep.addEventListener('click', () => {
        if (sim) {
            sim.tick(1);
            render();
            updateStats();
        }
    });

    speedSlider.addEventListener('input', () => {
        speed = parseInt(speedSlider.value);
        speedLabel.textContent = speed + 'x';
    });

    viewMode.addEventListener('change', () => {
        terrainImageData = null;
        renderTerrain();
        render();
    });

    showCreatures.addEventListener('change', () => render());

    canvas.addEventListener('click', (e) => {
        if (!sim) return;
        const rect = canvas.getBoundingClientRect();
        const x = Math.floor((e.clientX - rect.left) / SCALE);
        const y = Math.floor((e.clientY - rect.top) / SCALE);
        showTileInfo(x, y);

        // Switch to tile tab
        tabButtons.forEach(b => b.classList.remove('active'));
        document.querySelectorAll('.tab-content').forEach(t => t.classList.remove('active'));
        document.querySelector('[data-tab="tile-tab"]').classList.add('active');
        document.getElementById('tile-tab').classList.add('active');
    });

    canvas.addEventListener('mousemove', (e) => {
        if (!sim) return;
        const rect = canvas.getBoundingClientRect();
        const x = Math.floor((e.clientX - rect.left) / SCALE);
        const y = Math.floor((e.clientY - rect.top) / SCALE);
        canvas.title = `(${x}, ${y})`;
    });

    tabButtons.forEach(btn => {
        btn.addEventListener('click', () => {
            tabButtons.forEach(b => b.classList.remove('active'));
            document.querySelectorAll('.tab-content').forEach(t => t.classList.remove('active'));
            btn.classList.add('active');
            document.getElementById(btn.dataset.tab).classList.add('active');
        });
    });
}

// ===== Game Loop =====
function gameLoop() {
    if (!playing || !sim) return;

    sim.tick(speed);
    render();

    if (sim.get_tick() % 10 === 0) {
        updateStats();
    }

    animFrameId = requestAnimationFrame(gameLoop);
}

// ===== Rendering =====
function renderTerrain() {
    if (!sim) return;

    const w = sim.get_width();
    const h = sim.get_height();

    canvas.width = w * SCALE;
    canvas.height = h * SCALE;

    const mode = viewMode.value;

    if (mode === 'biome') {
        const rgba = sim.get_terrain_rgba();
        const imgData = new ImageData(new Uint8ClampedArray(rgba), w, h);
        // Scale up
        const offscreen = new OffscreenCanvas(w, h);
        const offCtx = offscreen.getContext('2d');
        offCtx.putImageData(imgData, 0, 0);
        terrainImageData = offscreen;
    } else {
        // Generate custom view
        const tileInfo = [];
        for (let y = 0; y < h; y++) {
            for (let x = 0; x < w; x++) {
                const raw = sim.get_tile_info(x, y);
                tileInfo.push(JSON.parse(raw));
            }
        }

        const offscreen = new OffscreenCanvas(w, h);
        const offCtx = offscreen.getContext('2d');
        const imgData = offCtx.createImageData(w, h);

        for (let y = 0; y < h; y++) {
            for (let x = 0; x < w; x++) {
                const info = tileInfo[y * w + x];
                const idx = (y * w + x) * 4;
                let r, g, b;

                if (mode === 'altitude') {
                    const alt = parseFloat(info.altitude);
                    if (alt <= 0) {
                        const d = Math.max(0, 1 + alt / 15000);
                        r = Math.floor(10 * d);
                        g = Math.floor(40 * d);
                        b = Math.floor(80 + 100 * d);
                    } else {
                        const d = alt / 15000;
                        r = Math.floor(50 + 180 * d);
                        g = Math.floor(120 + 100 * d);
                        b = Math.floor(50 * (1 - d));
                    }
                } else if (mode === 'temperature') {
                    const temp = parseFloat(info.temperature);
                    const t = (temp + 35) / 65; // normalize -35..30 to 0..1
                    r = Math.floor(255 * Math.min(1, t * 2));
                    g = Math.floor(255 * (1 - Math.abs(t - 0.5) * 2));
                    b = Math.floor(255 * Math.min(1, (1 - t) * 2));
                } else if (mode === 'rainfall') {
                    const rain = parseFloat(info.rainfall);
                    const d = rain / 13000;
                    r = Math.floor(200 * (1 - d));
                    g = Math.floor(180 * (1 - d) + 50);
                    b = Math.floor(80 + 175 * d);
                }

                imgData.data[idx] = r;
                imgData.data[idx + 1] = g;
                imgData.data[idx + 2] = b;
                imgData.data[idx + 3] = 255;
            }
        }

        offCtx.putImageData(imgData, 0, 0);
        terrainImageData = offscreen;
    }
}

function render() {
    if (!sim || !terrainImageData) return;

    const w = sim.get_width();
    const h = sim.get_height();

    ctx.imageSmoothingEnabled = false;
    ctx.drawImage(terrainImageData, 0, 0, w * SCALE, h * SCALE);

    // Draw creatures
    if (showCreatures.checked) {
        const data = sim.get_creature_data();
        const count = data.length / 8;

        for (let i = 0; i < count; i++) {
            const offset = i * 8;
            const x = data[offset];
            const y = data[offset + 1];
            const r = data[offset + 2];
            const g = data[offset + 3];
            const b = data[offset + 4];
            const size = data[offset + 5];
            const diet = data[offset + 6];

            const px = x * SCALE;
            const py = y * SCALE;
            const radius = Math.max(1.5, (size / 5) * SCALE * 0.5);

            ctx.fillStyle = `rgb(${r},${g},${b})`;
            ctx.beginPath();
            ctx.arc(px, py, radius, 0, Math.PI * 2);
            ctx.fill();

            // Carnivores get a small outline
            if (diet > 0.6) {
                ctx.strokeStyle = 'rgba(255,50,50,0.5)';
                ctx.lineWidth = 0.5;
                ctx.stroke();
            }
        }
    }
}

// ===== Stats & UI Updates =====
function updateStats() {
    if (!sim) return;

    const tick = sim.get_tick();
    const pop = sim.get_population();
    const specCount = sim.get_species_count();

    tickDisplay.textContent = `Tick: ${tick}`;
    popDisplay.textContent = `Pop: ${pop}`;
    speciesDisplay.textContent = `Species: ${specCount}`;

    updateSpeciesList();
    updateGraph();
}

function updateSpeciesList() {
    if (!sim) return;

    const stats = JSON.parse(sim.get_stats());
    const species = stats.species || [];

    // Sort by population descending
    species.sort((a, b) => b.population - a.population);

    speciesList.innerHTML = species.map(s => {
        const [r, g, b] = s.color;
        const dietClass = s.diet.toLowerCase();
        return `
            <div class="species-entry">
                <div class="species-color" style="background:rgb(${r},${g},${b})"></div>
                <span class="species-name">${s.name}</span>
                <span class="species-diet ${dietClass}">${s.diet}</span>
                <span class="species-pop">${s.population} (peak: ${s.peak_population})</span>
            </div>
        `;
    }).join('');
}

function showTileInfo(x, y) {
    if (!sim) return;

    const info = JSON.parse(sim.get_tile_info(x, y));
    if (!info.biome) {
        tileInfo.innerHTML = '<p class="hint">No data for this tile.</p>';
        return;
    }

    let html = `
        <div class="tile-detail">
            <h4>Terrain (${info.x}, ${info.y})</h4>
            <div class="tile-row"><span class="label">Biome</span><span class="value">${info.biome}</span></div>
            <div class="tile-row"><span class="label">Altitude</span><span class="value">${info.altitude} m</span></div>
            <div class="tile-row"><span class="label">Temperature</span><span class="value">${info.temperature} &deg;C</span></div>
            <div class="tile-row"><span class="label">Rainfall</span><span class="value">${info.rainfall} mm</span></div>
            <div class="tile-row"><span class="label">Plant Biomass</span><span class="value">${info.plant_biomass} / ${info.max_biomass}</span></div>
            <div class="tile-row"><span class="label">Creatures</span><span class="value">${info.creature_count}</span></div>
        </div>
    `;

    if (info.biome_presences && info.biome_presences.length > 0) {
        html += '<div class="tile-detail"><h4>Biome Composition</h4>';
        info.biome_presences.forEach(bp => {
            html += `<div class="tile-row"><span class="label">${bp.biome}</span><span class="value">${bp.presence}</span></div>`;
        });
        html += '</div>';
    }

    if (info.creatures && info.creatures.length > 0) {
        html += '<div class="tile-detail"><h4>Creatures</h4>';
        info.creatures.forEach(c => {
            html += `
                <div class="creature-card">
                    <div class="creature-header">
                        <span>${c.species} (#${c.id})</span>
                        <span>${c.diet_label}</span>
                    </div>
                    <div class="creature-stats">
                        <div class="stat"><span class="label">Age</span><span>${c.age}</span></div>
                        <div class="stat"><span class="label">Gen</span><span>${c.generation}</span></div>
                        <div class="stat"><span class="label">Energy</span><span>${c.energy}</span></div>
                        <div class="stat"><span class="label">Health</span><span>${c.health}</span></div>
                        <div class="stat"><span class="label">Size</span><span>${c.size}</span></div>
                        <div class="stat"><span class="label">Speed</span><span>${c.speed}</span></div>
                        <div class="stat"><span class="label">Diet</span><span>${c.diet}</span></div>
                        <div class="stat"><span class="label">Camo</span><span>${c.camouflage}</span></div>
                        <div class="stat"><span class="label">Cold Tol</span><span>${c.cold_tol}&deg;</span></div>
                        <div class="stat"><span class="label">Heat Tol</span><span>${c.heat_tol}&deg;</span></div>
                        <div class="stat"><span class="label">Kills</span><span>${c.kills}</span></div>
                        <div class="stat"><span class="label">Children</span><span>${c.children}</span></div>
                        <div class="stat"><span class="label">Activity</span><span>${c.activity}</span></div>
                    </div>
                </div>
            `;
        });
        html += '</div>';
    }

    tileInfo.innerHTML = html;
}

// ===== Population Graph =====
function updateGraph() {
    if (!sim) return;

    const history = JSON.parse(sim.get_history());
    if (history.length < 2) return;

    const gw = graphCanvas.width;
    const gh = graphCanvas.height;
    const padding = { top: 15, right: 10, bottom: 25, left: 40 };
    const plotW = gw - padding.left - padding.right;
    const plotH = gh - padding.top - padding.bottom;

    graphCtx.clearRect(0, 0, gw, gh);
    graphCtx.fillStyle = '#111';
    graphCtx.fillRect(0, 0, gw, gh);

    // Find ranges
    const maxPop = Math.max(10, ...history.map(h => h.total));
    const minTick = history[0].tick;
    const maxTick = history[history.length - 1].tick;
    const tickRange = Math.max(1, maxTick - minTick);

    // Grid lines
    graphCtx.strokeStyle = '#333';
    graphCtx.lineWidth = 0.5;
    for (let i = 0; i <= 4; i++) {
        const y = padding.top + (plotH * i) / 4;
        graphCtx.beginPath();
        graphCtx.moveTo(padding.left, y);
        graphCtx.lineTo(padding.left + plotW, y);
        graphCtx.stroke();

        graphCtx.fillStyle = '#666';
        graphCtx.font = '9px sans-serif';
        graphCtx.textAlign = 'right';
        graphCtx.fillText(Math.round(maxPop * (1 - i / 4)), padding.left - 4, y + 3);
    }

    // Helper to map data to canvas
    const toX = (tick) => padding.left + ((tick - minTick) / tickRange) * plotW;
    const toY = (val) => padding.top + plotH - (val / maxPop) * plotH;

    // Draw lines
    const lines = [
        { key: 'total', color: '#53d8fb', label: 'Total' },
        { key: 'herbivores', color: '#4caf50', label: 'Herbivores' },
        { key: 'carnivores', color: '#f44336', label: 'Carnivores' },
        { key: 'species_count', color: '#ff9800', label: 'Species', scale: maxPop / Math.max(1, ...history.map(h => h.species_count)) },
    ];

    lines.forEach(line => {
        graphCtx.strokeStyle = line.color;
        graphCtx.lineWidth = 1.5;
        graphCtx.beginPath();
        history.forEach((h, idx) => {
            const val = line.scale ? h[line.key] * line.scale : h[line.key];
            const x = toX(h.tick);
            const y = toY(val);
            if (idx === 0) graphCtx.moveTo(x, y);
            else graphCtx.lineTo(x, y);
        });
        graphCtx.stroke();
    });

    // Tick axis label
    graphCtx.fillStyle = '#666';
    graphCtx.font = '9px sans-serif';
    graphCtx.textAlign = 'center';
    graphCtx.fillText(`Tick ${minTick}`, padding.left, gh - 4);
    graphCtx.fillText(`Tick ${maxTick}`, padding.left + plotW, gh - 4);

    // Legend
    const legendDiv = document.getElementById('graph-legend');
    legendDiv.innerHTML = lines.map(l =>
        `<div class="legend-item">
            <div class="legend-color" style="background:${l.color}"></div>
            <span>${l.label}${l.scale ? ' (scaled)' : ''}</span>
        </div>`
    ).join('');
}

// ===== Start =====
main().catch(console.error);
