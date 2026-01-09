// Civilization History Simulator - Main JavaScript

let wasm = null;
let simulation = null;
let isRunning = false;
let animationId = null;
let speed = 10;
let viewMode = 'political';
let worldData = null;
let civilizations = [];

// Canvas and rendering
let mapCanvas, mapCtx;
let minimapCanvas, minimapCtx;
let scale = 4;
let offsetX = 0;
let offsetY = 0;
let isDragging = false;
let dragStart = { x: 0, y: 0 };

// Initialize the application
async function init() {
    try {
        // Import WASM module
        wasm = await import('./pkg/civ_sim.js');
        await wasm.default();

        console.log('WASM module loaded');

        // Setup canvas
        mapCanvas = document.getElementById('map-canvas');
        mapCtx = mapCanvas.getContext('2d');
        minimapCanvas = document.getElementById('minimap-canvas');
        minimapCtx = minimapCanvas.getContext('2d');

        // Setup event listeners
        setupEventListeners();

        // Generate initial world
        generateWorld();

    } catch (error) {
        console.error('Failed to initialize:', error);
        showError('Failed to load simulation. Please refresh the page.');
    }
}

function setupEventListeners() {
    // Control buttons
    document.getElementById('btn-new').addEventListener('click', generateWorld);
    document.getElementById('btn-play').addEventListener('click', togglePlay);
    document.getElementById('btn-step').addEventListener('click', stepSimulation);
    document.getElementById('btn-fast').addEventListener('click', () => advanceYears(100));

    // Speed select
    document.getElementById('speed-select').addEventListener('change', (e) => {
        speed = parseInt(e.target.value);
    });

    // View select
    document.getElementById('view-select').addEventListener('change', (e) => {
        viewMode = e.target.value;
        render();
    });

    // Map interactions
    mapCanvas.addEventListener('mousedown', onMapMouseDown);
    mapCanvas.addEventListener('mousemove', onMapMouseMove);
    mapCanvas.addEventListener('mouseup', onMapMouseUp);
    mapCanvas.addEventListener('mouseleave', onMapMouseUp);
    mapCanvas.addEventListener('wheel', onMapWheel);
    mapCanvas.addEventListener('click', onMapClick);

    // Window resize
    window.addEventListener('resize', resizeCanvas);

    // Modal close
    document.querySelector('.close-btn').addEventListener('click', closeModal);
    document.getElementById('modal').addEventListener('click', (e) => {
        if (e.target.id === 'modal') closeModal();
    });
}

function generateWorld() {
    const seed = Math.floor(Math.random() * 2147483647);
    const width = 400;
    const height = 200;

    showProgress('Generating world...');

    setTimeout(() => {
        try {
            wasm.init_simulation(width, height, seed);

            // Load world data
            worldData = JSON.parse(wasm.get_world_data());

            // Setup canvas size
            resizeCanvas();

            // Initial render
            render();
            updateUI();

            hideProgress();
            console.log('World generated with seed:', seed);

        } catch (error) {
            console.error('Failed to generate world:', error);
            hideProgress();
            showError('Failed to generate world.');
        }
    }, 100);
}

function togglePlay() {
    isRunning = !isRunning;
    const btn = document.getElementById('btn-play');
    btn.textContent = isRunning ? 'Pause' : 'Play';

    if (isRunning) {
        runSimulation();
    } else {
        cancelAnimationFrame(animationId);
    }
}

function runSimulation() {
    if (!isRunning) return;

    for (let i = 0; i < speed; i++) {
        try {
            wasm.advance_simulation(1);
        } catch (error) {
            console.error('Simulation error:', error);
            isRunning = false;
            document.getElementById('btn-play').textContent = 'Play';
            return;
        }
    }

    updateUI();
    render();

    animationId = requestAnimationFrame(runSimulation);
}

function stepSimulation() {
    try {
        wasm.advance_simulation(1);
        updateUI();
        render();
    } catch (error) {
        console.error('Step error:', error);
    }
}

function advanceYears(years) {
    showProgress(`Advancing ${years} years...`);

    setTimeout(() => {
        try {
            wasm.advance_simulation(years);
            updateUI();
            render();
            hideProgress();
        } catch (error) {
            console.error('Advance error:', error);
            hideProgress();
        }
    }, 10);
}

function updateUI() {
    try {
        // Update year
        const year = wasm.get_current_year();
        document.getElementById('current-year').textContent = formatYear(year);

        // Update statistics
        const stats = JSON.parse(wasm.get_statistics());
        document.getElementById('stat-population').textContent = formatNumber(stats.total_population);
        document.getElementById('stat-nations').textContent = stats.total_nations;
        document.getElementById('stat-settlements').textContent = stats.total_settlements;
        document.getElementById('stat-cultures').textContent = stats.total_cultures;
        document.getElementById('stat-religions').textContent = stats.total_religions;
        document.getElementById('stat-wars').textContent = stats.wars_fought;

        // Update civilizations list
        civilizations = JSON.parse(wasm.get_civilizations_data());
        updateNationsList();

        // Update history
        const history = JSON.parse(wasm.get_history_data());
        updateHistoryList(history);

    } catch (error) {
        console.error('UI update error:', error);
    }
}

function updateNationsList() {
    const container = document.getElementById('nations-list');

    // Sort by population
    const sortedNations = [...civilizations].sort((a, b) => b.population - a.population);
    const topNations = sortedNations.slice(0, 10);

    container.innerHTML = topNations.map(nation => `
        <div class="nation-item" data-id="${nation.id}">
            <div class="nation-color" style="background: rgb(${nation.color[0]}, ${nation.color[1]}, ${nation.color[2]})"></div>
            <span class="nation-name">${nation.name}</span>
            <span class="nation-pop">${formatNumber(nation.population)}</span>
        </div>
    `).join('');

    // Add click handlers
    container.querySelectorAll('.nation-item').forEach(item => {
        item.addEventListener('click', () => {
            const nationId = parseInt(item.dataset.id);
            showNationDetails(nationId);
        });
    });
}

function updateHistoryList(history) {
    const container = document.getElementById('history-list');

    container.innerHTML = history.slice(0, 20).map(event => `
        <div class="history-item">
            <span class="history-year">${formatYear(event.year)}</span>
            ${event.description}
        </div>
    `).join('');
}

function render() {
    if (!worldData) return;

    const width = worldData.width;
    const height = worldData.height;

    // Create image data
    const imageData = mapCtx.createImageData(width, height);

    // Build nation color map
    const nationColors = new Map();
    civilizations.forEach(civ => {
        nationColors.set(civ.id, civ.color);
    });

    // Biome colors
    const biomeColors = {
        'Ocean': [28, 66, 120],
        'DeepOcean': [15, 40, 90],
        'IceCap': [240, 248, 255],
        'Tundra': [139, 139, 128],
        'Taiga': [43, 63, 40],
        'Grassland': [154, 205, 50],
        'Savanna': [210, 180, 90],
        'Desert': [253, 225, 171],
        'Forest': [34, 139, 34],
        'Rainforest': [0, 100, 0],
        'Mountains': [139, 137, 137],
        'Hills': [107, 142, 35],
        'Wetlands': [47, 79, 79],
        'CoastalWaters': [64, 164, 223]
    };

    // Render each tile
    for (const tile of worldData.tiles) {
        const idx = (tile.y * width + tile.x) * 4;
        let color;

        switch (viewMode) {
            case 'political':
                if (tile.owner_nation_id && nationColors.has(tile.owner_nation_id)) {
                    color = nationColors.get(tile.owner_nation_id);
                } else {
                    color = biomeColors[tile.biome] || [50, 50, 50];
                }
                break;
            case 'biomes':
                color = biomeColors[tile.biome] || [50, 50, 50];
                break;
            case 'terrain':
                const alt = (tile.altitude + 1) / 2;
                if (tile.altitude < 0) {
                    const depth = Math.min(Math.abs(tile.altitude) * 100, 255);
                    color = [0, Math.max(50, 100 - depth), Math.max(150, 200 - depth)];
                } else {
                    color = [
                        Math.min(100 + alt * 80, 255),
                        Math.min(80 + alt * 50, 200),
                        Math.min(60, 100)
                    ];
                }
                break;
            case 'temperature':
                const temp = (tile.altitude >= 0) ? 0.5 : 0.3; // Simplified
                color = [temp * 255, 50, (1 - temp) * 255];
                break;
            case 'rainfall':
                color = biomeColors[tile.biome] || [50, 50, 50]; // Use biome as proxy
                break;
            case 'population':
                if (tile.settlement_id) {
                    color = [255, 100, 50];
                } else {
                    color = biomeColors[tile.biome] || [50, 50, 50];
                }
                break;
            default:
                color = biomeColors[tile.biome] || [50, 50, 50];
        }

        imageData.data[idx] = color[0];
        imageData.data[idx + 1] = color[1];
        imageData.data[idx + 2] = color[2];
        imageData.data[idx + 3] = 255;
    }

    // Draw to offscreen canvas first
    const offscreen = new OffscreenCanvas(width, height);
    const offCtx = offscreen.getContext('2d');
    offCtx.putImageData(imageData, 0, 0);

    // Clear and draw scaled
    mapCtx.imageSmoothingEnabled = false;
    mapCtx.clearRect(0, 0, mapCanvas.width, mapCanvas.height);
    mapCtx.drawImage(
        offscreen,
        offsetX, offsetY,
        width * scale, height * scale
    );

    // Draw settlements
    drawSettlements();

    // Update minimap
    renderMinimap(imageData, width, height);
}

function drawSettlements() {
    if (!worldData) return;

    mapCtx.fillStyle = '#fff';
    mapCtx.strokeStyle = '#000';
    mapCtx.lineWidth = 1;

    for (const tile of worldData.tiles) {
        if (tile.settlement_id) {
            const x = tile.x * scale + offsetX;
            const y = tile.y * scale + offsetY;

            // Draw settlement marker
            mapCtx.beginPath();
            mapCtx.arc(x + scale/2, y + scale/2, Math.max(2, scale/3), 0, Math.PI * 2);
            mapCtx.fill();
            mapCtx.stroke();
        }
    }
}

function renderMinimap(imageData, width, height) {
    const mmWidth = 200;
    const mmHeight = 100;

    minimapCanvas.width = mmWidth;
    minimapCanvas.height = mmHeight;

    // Create scaled image
    const offscreen = new OffscreenCanvas(width, height);
    const offCtx = offscreen.getContext('2d');
    offCtx.putImageData(imageData, 0, 0);

    minimapCtx.imageSmoothingEnabled = false;
    minimapCtx.drawImage(offscreen, 0, 0, mmWidth, mmHeight);

    // Draw viewport rectangle
    if (worldData) {
        const viewX = (-offsetX / scale / worldData.width) * mmWidth;
        const viewY = (-offsetY / scale / worldData.height) * mmHeight;
        const viewW = (mapCanvas.width / scale / worldData.width) * mmWidth;
        const viewH = (mapCanvas.height / scale / worldData.height) * mmHeight;

        minimapCtx.strokeStyle = '#e94560';
        minimapCtx.lineWidth = 2;
        minimapCtx.strokeRect(viewX, viewY, viewW, viewH);
    }
}

function resizeCanvas() {
    const container = document.getElementById('map-container');
    mapCanvas.width = container.clientWidth;
    mapCanvas.height = container.clientHeight;

    render();
}

// Map interaction handlers
function onMapMouseDown(e) {
    isDragging = true;
    dragStart = { x: e.clientX - offsetX, y: e.clientY - offsetY };
    mapCanvas.style.cursor = 'grabbing';
}

function onMapMouseMove(e) {
    if (isDragging) {
        offsetX = e.clientX - dragStart.x;
        offsetY = e.clientY - dragStart.y;
        render();
    } else {
        // Update tile info
        updateTileInfo(e);
    }
}

function onMapMouseUp() {
    isDragging = false;
    mapCanvas.style.cursor = 'crosshair';
}

function onMapWheel(e) {
    e.preventDefault();

    const rect = mapCanvas.getBoundingClientRect();
    const mouseX = e.clientX - rect.left;
    const mouseY = e.clientY - rect.top;

    const oldScale = scale;
    if (e.deltaY < 0) {
        scale = Math.min(scale * 1.2, 20);
    } else {
        scale = Math.max(scale / 1.2, 1);
    }

    // Zoom towards mouse position
    offsetX = mouseX - (mouseX - offsetX) * (scale / oldScale);
    offsetY = mouseY - (mouseY - offsetY) * (scale / oldScale);

    render();
}

function onMapClick(e) {
    if (!worldData) return;

    const rect = mapCanvas.getBoundingClientRect();
    const x = Math.floor((e.clientX - rect.left - offsetX) / scale);
    const y = Math.floor((e.clientY - rect.top - offsetY) / scale);

    if (x >= 0 && x < worldData.width && y >= 0 && y < worldData.height) {
        showTileDetails(x, y);
    }
}

function updateTileInfo(e) {
    if (!worldData) return;

    const rect = mapCanvas.getBoundingClientRect();
    const x = Math.floor((e.clientX - rect.left - offsetX) / scale);
    const y = Math.floor((e.clientY - rect.top - offsetY) / scale);

    if (x >= 0 && x < worldData.width && y >= 0 && y < worldData.height) {
        try {
            const tileInfo = JSON.parse(wasm.get_tile_info(x, y));

            let html = `
                <p><span class="label">Position:</span> <span class="value">${x}, ${y}</span></p>
                <p><span class="label">Biome:</span> <span class="value">${tileInfo.biome}</span></p>
                <p><span class="label">Altitude:</span> <span class="value">${tileInfo.altitude.toFixed(2)}</span></p>
            `;

            if (tileInfo.is_river) html += `<p><span class="value">River</span></p>`;
            if (tileInfo.is_coastal) html += `<p><span class="value">Coastal</span></p>`;

            if (tileInfo.settlement_name) {
                html += `<p><span class="label">Settlement:</span> <span class="value">${tileInfo.settlement_name}</span></p>`;
                html += `<p><span class="label">Population:</span> <span class="value">${formatNumber(tileInfo.settlement_population)}</span></p>`;
            }

            if (tileInfo.nation_name) {
                html += `<p><span class="label">Nation:</span> <span class="value">${tileInfo.nation_name}</span></p>`;
            }

            if (tileInfo.resources && tileInfo.resources.length > 0) {
                html += `<p><span class="label">Resources:</span> <span class="value">${tileInfo.resources.join(', ')}</span></p>`;
            }

            document.getElementById('tile-info').innerHTML = html;
        } catch (error) {
            // Ignore errors
        }
    }
}

function showTileDetails(x, y) {
    try {
        const tileInfo = JSON.parse(wasm.get_tile_info(x, y));

        let html = `<h3>Tile (${x}, ${y})</h3>`;
        html += `<p><strong>Biome:</strong> ${tileInfo.biome}</p>`;
        html += `<p><strong>Altitude:</strong> ${tileInfo.altitude.toFixed(2)}</p>`;
        html += `<p><strong>Temperature:</strong> ${tileInfo.temperature.toFixed(1)}°C</p>`;
        html += `<p><strong>Rainfall:</strong> ${tileInfo.rainfall.toFixed(0)} mm</p>`;

        if (tileInfo.settlement_name) {
            html += `<hr><h4>${tileInfo.settlement_name}</h4>`;
            html += `<p><strong>Population:</strong> ${formatNumber(tileInfo.settlement_population)}</p>`;
        }

        if (tileInfo.nation_name) {
            html += `<p><strong>Controlled by:</strong> ${tileInfo.nation_name}</p>`;
        }

        if (tileInfo.resources.length > 0) {
            html += `<p><strong>Resources:</strong> ${tileInfo.resources.join(', ')}</p>`;
        }

        showModal(html);
    } catch (error) {
        console.error('Error showing tile details:', error);
    }
}

function showNationDetails(nationId) {
    const nation = civilizations.find(c => c.id === nationId);
    if (!nation) return;

    let html = `
        <h3>${nation.name}</h3>
        <p><strong>Government:</strong> ${nation.government}</p>
        <p><strong>Population:</strong> ${formatNumber(nation.population)}</p>
        <p><strong>Settlements:</strong> ${nation.settlements}</p>
        <p><strong>Territory:</strong> ${nation.territory} tiles</p>
        <p><strong>Treasury:</strong> ${formatNumber(Math.floor(nation.treasury))} gold</p>
        <p><strong>Military Strength:</strong> ${formatNumber(Math.floor(nation.military_strength))}</p>
    `;

    showModal(html);
}

// Utility functions
function formatYear(year) {
    if (year < 0) {
        return `${Math.abs(year)} BCE`;
    }
    return `${year} CE`;
}

function formatNumber(n) {
    if (n >= 1000000000) return (n / 1000000000).toFixed(1) + 'B';
    if (n >= 1000000) return (n / 1000000).toFixed(1) + 'M';
    if (n >= 1000) return (n / 1000).toFixed(1) + 'K';
    return n.toString();
}

function showProgress(text) {
    const container = document.getElementById('progress-container');
    document.getElementById('progress-text').textContent = text;
    container.style.display = 'flex';
}

function hideProgress() {
    document.getElementById('progress-container').style.display = 'none';
}

function showModal(content) {
    document.getElementById('modal-body').innerHTML = content;
    document.getElementById('modal').style.display = 'flex';
}

function closeModal() {
    document.getElementById('modal').style.display = 'none';
}

function showError(message) {
    showModal(`<h3 style="color: #e94560;">Error</h3><p>${message}</p>`);
}

// Initialize when DOM is ready
document.addEventListener('DOMContentLoaded', init);
