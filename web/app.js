document.addEventListener('DOMContentLoaded', () => {
    const API_BASE = 'http://localhost:3030/api';

    // Elements
    const agentStatusBadge = document.getElementById('agent-status-badge');
    const licenseBadge = document.getElementById('license-badge');
    const cpuUsageVal = document.getElementById('cpu-usage-val');
    const cpuProgress = document.getElementById('cpu-progress');
    const cpuCoreCount = document.getElementById('cpu-core-count');

    const ramUsageVal = document.getElementById('ram-usage-val');
    const ramProgress = document.getElementById('ram-progress');
    const ramDetail = document.getElementById('ram-detail');

    const osInfo = document.getElementById('os-info');
    const disksContainer = document.getElementById('disks-container');
    const processesTbody = document.getElementById('processes-tbody');
    const consoleOutput = document.getElementById('console-output');

    // AI Chat & Log Analyzer Elements
    const chatMessages = document.getElementById('chat-messages');
    const chatInput = document.getElementById('chat-input');
    const btnSendChat = document.getElementById('btn-send-chat');
    const btnAnalyzeLogs = document.getElementById('btn-analyze-logs');
    const logAnalysisResult = document.getElementById('log-analysis-result');

    // Modal Elements
    const btnOpenLicense = document.getElementById('btn-open-license');
    const btnCloseModal = document.getElementById('btn-close-modal');
    const licenseModal = document.getElementById('license-modal');
    const licenseKeyInput = document.getElementById('license-key-input');
    const btnActivateLicense = document.getElementById('btn-activate-license');
    const licenseFeedback = document.getElementById('license-feedback');

    // Modal Tabs Elements
    const tabBtnActivate = document.getElementById('tab-btn-activate');
    const tabBtnGenerate = document.getElementById('tab-btn-generate');
    const tabActivateContainer = document.getElementById('tab-activate-container');
    const tabGenerateContainer = document.getElementById('tab-generate-container');

    // Key Generator Elements
    const genClientInput = document.getElementById('gen-client-input');
    const genExpiryInput = document.getElementById('gen-expiry-input');
    const btnGenerateKey = document.getElementById('btn-generate-key');
    const genOutputBox = document.getElementById('gen-output-box');
    const genKeyResult = document.getElementById('gen-key-result');
    const btnAutoApply = document.getElementById('btn-auto-apply');

    // Buttons
    const btnRefresh = document.getElementById('btn-refresh');
    const btnFixNetwork = document.getElementById('btn-fix-network');
    const btnFixSpooler = document.getElementById('btn-fix-spooler');
    const btnFixSfc = document.getElementById('btn-fix-sfc');
    const btnFixGpupdate = document.getElementById('btn-fix-gpupdate');
    const btnClearLog = document.getElementById('btn-clear-log');

    function appendLog(message, type = 'info') {
        const timeStr = new Date().toLocaleTimeString();
        const div = document.createElement('div');
        div.className = `log-line log-${type}`;
        div.textContent = `[${timeStr}] ${message}`;
        consoleOutput.appendChild(div);
        consoleOutput.scrollTop = consoleOutput.scrollHeight;
    }

    // Fetch License Status
    async function fetchLicenseStatus() {
        try {
            const res = await fetch(`${API_BASE}/license/status`);
            const data = await res.json();
            if (data.is_pro) {
                licenseBadge.className = 'badge badge-license-pro';
                licenseBadge.textContent = `👑 PRO: ${data.client_name}`;
            } else {
                licenseBadge.className = 'badge badge-license';
                licenseBadge.textContent = 'COMMUNITY EDITION';
            }
        } catch (err) {
            console.error('License fetch error:', err);
        }
    }

    // Fetch System Telemetry
    async function fetchSystemStatus() {
        try {
            const res = await fetch(`${API_BASE}/status`);
            if (!res.ok) throw new Error(`HTTP Error ${res.status}`);
            const data = await res.json();

            agentStatusBadge.className = 'badge badge-online';
            agentStatusBadge.textContent = '● Agent Online';

            const m = data.metrics;
            cpuUsageVal.textContent = `${m.cpu_global_usage}%`;
            cpuProgress.style.width = `${Math.min(m.cpu_global_usage, 100)}%`;
            cpuCoreCount.textContent = `${m.cpu_count} Cores`;

            ramUsageVal.textContent = `${m.memory_used_percent}%`;
            ramProgress.style.width = `${Math.min(m.memory_used_percent, 100)}%`;
            ramDetail.textContent = `${m.used_memory_mb} / ${m.total_memory_mb} MB`;

            osInfo.textContent = `${m.os_name} ${m.os_version}`;

            disksContainer.innerHTML = '';
            if (m.disks && m.disks.length > 0) {
                m.disks.forEach(d => {
                    const freePercent = ((d.available_gb / d.total_gb) * 100).toFixed(1);
                    const item = document.createElement('div');
                    item.className = 'disk-item';
                    item.innerHTML = `
                        <span>Drive <strong>${d.mount_point || d.name}</strong></span>
                        <span>${d.available_gb} GB free of ${d.total_gb} GB (${freePercent}%)</span>
                    `;
                    disksContainer.appendChild(item);
                });
            } else {
                disksContainer.innerHTML = '<p class="text-muted">No disk info available</p>';
            }

            processesTbody.innerHTML = '';
            if (data.top_processes && data.top_processes.length > 0) {
                data.top_processes.forEach(p => {
                    const tr = document.createElement('tr');
                    tr.innerHTML = `
                        <td><code>${p.pid}</code></td>
                        <td><strong>${escapeHtml(p.name)}</strong></td>
                        <td>${p.cpu_usage}%</td>
                        <td>${p.memory_mb} MB</td>
                        <td>
                            <button class="btn btn-sm btn-danger btn-kill" data-pid="${p.pid}" data-name="${escapeHtml(p.name)}">
                                Kill App
                            </button>
                        </td>
                    `;
                    processesTbody.appendChild(tr);
                });

                document.querySelectorAll('.btn-kill').forEach(btn => {
                    btn.addEventListener('click', async (e) => {
                        const pid = parseInt(e.target.dataset.pid);
                        const name = e.target.dataset.name;
                        if (confirm(`Hentikan proses ${name} (PID: ${pid})?`)) {
                            await executeKillProcess(pid, name);
                        }
                    });
                });
            } else {
                processesTbody.innerHTML = '<tr><td colspan="5" class="text-center">Tidak ada proses terdeteksi</td></tr>';
            }

        } catch (err) {
            agentStatusBadge.className = 'badge badge-offline';
            agentStatusBadge.textContent = '● Agent Offline / Disconnected';
            console.error('Failed to fetch telemetry:', err);
        }
    }

    function escapeHtml(str) {
        return str.replace(/[&<>"']/g, function(m) {
            return {
                '&': '&amp;',
                '<': '&lt;',
                '>': '&gt;',
                '"': '&quot;',
                "'": '&#039;'
            }[m];
        });
    }

    async function triggerAction(endpoint, actionName) {
        appendLog(`[ACTION] Triggering ${actionName}...`, 'warn');
        try {
            const res = await fetch(`${API_BASE}/${endpoint}`, { method: 'POST' });
            const data = await res.json();
            if (data.success) {
                appendLog(`[SUCCESS] ${actionName} Selesai!\nSTDOUT:\n${data.stdout || '(no output)'}`, 'success');
            } else {
                appendLog(`[ERROR] ${actionName} Gagal!\nSTDERR:\n${data.stderr || '(unknown error)'}`, 'error');
            }
        } catch (err) {
            appendLog(`[EXCEPTION] ${actionName} Error: ${err.message}`, 'error');
        }
        fetchSystemStatus();
    }

    async function executeKillProcess(pid, name) {
        appendLog(`[ACTION] Terminating process ${name} (PID ${pid})...`, 'warn');
        try {
            const res = await fetch(`${API_BASE}/process/kill`, {
                method: 'POST',
                headers: { 'Content-Type': 'application/json' },
                body: JSON.stringify({ pid })
            });
            const data = await res.json();
            if (data.success) {
                appendLog(`[SUCCESS] Process ${name} (PID ${pid}) berhasil dihentikan.`, 'success');
            } else {
                appendLog(`[ERROR] Gagal menghentikan process ${name}: ${data.stderr}`, 'error');
            }
        } catch (err) {
            appendLog(`[EXCEPTION] Kill process error: ${err.message}`, 'error');
        }
        fetchSystemStatus();
    }

    // AI Chat Assistant Handlers
    async function sendChatMessage() {
        const msg = chatInput.value.trim();
        if (!msg) return;

        const userDiv = document.createElement('div');
        userDiv.className = 'chat-bubble user-bubble';
        userDiv.textContent = msg;
        chatMessages.appendChild(userDiv);

        chatInput.value = '';
        chatMessages.scrollTop = chatMessages.scrollHeight;

        const aiDiv = document.createElement('div');
        aiDiv.className = 'chat-bubble ai-bubble';
        aiDiv.textContent = '⚡ AI sedang menganalisis kendala Anda...';
        chatMessages.appendChild(aiDiv);
        chatMessages.scrollTop = chatMessages.scrollHeight;

        try {
            const res = await fetch(`${API_BASE}/ai/chat`, {
                method: 'POST',
                headers: { 'Content-Type': 'application/json' },
                body: JSON.stringify({ message: msg })
            });
            const data = await res.json();

            aiDiv.innerHTML = `<strong>Nantara AI:</strong> ${escapeHtml(data.explanation)}`;

            if (data.recommended_actions && data.recommended_actions.length > 0) {
                const actionBox = document.createElement('div');
                actionBox.style.marginTop = '10px';
                actionBox.innerHTML = '<small style="color: var(--text-muted); display: block; margin-bottom: 6px;">💡 Rekomendasi Aksi Perbaikan:</small>';

                data.recommended_actions.forEach(act => {
                    const btn = document.createElement('button');
                    btn.className = 'btn btn-sm btn-primary ai-action-btn';
                    btn.style.marginRight = '6px';
                    btn.textContent = `▶ ${act.label}`;
                    btn.addEventListener('click', () => {
                        if (act.id === 'fix_network') triggerAction('fix/network', 'Fix Network & DNS');
                        else if (act.id === 'fix_spooler') triggerAction('fix/spooler', 'Fix Printer Spooler');
                        else if (act.id === 'run_sfc') triggerAction('fix/sfc', 'System Repair (SFC)');
                        else if (act.id === 'run_gpupdate') triggerAction('fix/gpupdate', 'Group Policy Update');
                        else if (act.id === 'kill_heavy_apps') fetchSystemStatus();
                    });
                    actionBox.appendChild(btn);
                });
                aiDiv.appendChild(actionBox);
            }
        } catch (err) {
            aiDiv.textContent = '❌ Gagal terhubung dengan AI Engine.';
        }
        chatMessages.scrollTop = chatMessages.scrollHeight;
    }

    // AI Log Analyzer Handler
    async function analyzeLogs() {
        logAnalysisResult.innerHTML = '<p class="text-muted">🔍 Membaca Event Viewer Log & Memindai Minidump BSOD...</p>';
        try {
            const res = await fetch(`${API_BASE}/ai/analyze-logs`, { method: 'POST' });
            const data = await res.json();

            let html = `<p style="color: var(--accent-cyan); font-weight: 600; margin-bottom: 8px;">${escapeHtml(data.explanation)}</p>`;

            if (data.recommended_actions && data.recommended_actions.length > 0) {
                html += '<div style="margin-top: 10px;">';
                data.recommended_actions.forEach(act => {
                    html += `<button class="btn btn-sm btn-warning btn-ai-log-fix" data-id="${act.id}" style="margin-right: 6px; margin-top: 6px;">▶ ${escapeHtml(act.label)}</button>`;
                });
                html += '</div>';
            }

            logAnalysisResult.innerHTML = html;

            document.querySelectorAll('.btn-ai-log-fix').forEach(btn => {
                btn.addEventListener('click', (e) => {
                    const id = e.target.dataset.id;
                    if (id === 'fix_network') triggerAction('fix/network', 'Fix Network & DNS');
                    else if (id === 'fix_spooler') triggerAction('fix/spooler', 'Fix Printer Spooler');
                    else if (id === 'run_sfc') triggerAction('fix/sfc', 'System Repair (SFC)');
                });
            });

        } catch (err) {
            logAnalysisResult.innerHTML = '<p class="log-error">❌ Gagal melakukan analisis log.</p>';
        }
    }

    // License Modal & Tab Handlers
    btnOpenLicense.addEventListener('click', () => {
        licenseModal.style.display = 'flex';
        licenseFeedback.textContent = '';
    });

    btnCloseModal.addEventListener('click', () => {
        licenseModal.style.display = 'none';
    });

    tabBtnActivate.addEventListener('click', () => {
        tabBtnActivate.className = 'btn btn-sm btn-primary';
        tabBtnGenerate.className = 'btn btn-sm btn-secondary';
        tabActivateContainer.style.display = 'block';
        tabGenerateContainer.style.display = 'none';
    });

    tabBtnGenerate.addEventListener('click', () => {
        tabBtnGenerate.className = 'btn btn-sm btn-primary';
        tabBtnActivate.className = 'btn btn-sm btn-secondary';
        tabGenerateContainer.style.display = 'block';
        tabActivateContainer.style.display = 'none';
    });

    // Generate Pro Key Handler
    btnGenerateKey.addEventListener('click', async () => {
        const client = genClientInput.value.trim() || 'PT Nantara Digital';
        const expiry = genExpiryInput.value.trim() || '2027-12-31';

        try {
            const res = await fetch(`${API_BASE}/license/generate`, {
                method: 'POST',
                headers: { 'Content-Type': 'application/json' },
                body: JSON.stringify({ client_name: client, expiry_date: expiry })
            });
            const data = await res.json();
            genKeyResult.value = data.pro_key;
            genOutputBox.style.display = 'block';
        } catch (err) {
            alert('Gagal menghasilkan Pro License Key.');
        }
    });

    // Auto Apply Generated License
    btnAutoApply.addEventListener('click', async () => {
        const key = genKeyResult.value.trim();
        if (!key) return;

        try {
            const res = await fetch(`${API_BASE}/license/activate`, {
                method: 'POST',
                headers: { 'Content-Type': 'application/json' },
                body: JSON.stringify({ license_key: key })
            });
            const data = await res.json();
            if (data.is_pro) {
                alert(`✅ Lisensi Pro berhasil diaktifkan untuk ${data.client_name}!`);
                fetchLicenseStatus();
                licenseModal.style.display = 'none';
            }
        } catch (err) {
            alert('Gagal mengaktifkan lisensi.');
        }
    });

    // Manual Activation Handler
    btnActivateLicense.addEventListener('click', async () => {
        const key = licenseKeyInput.value.trim();
        if (!key) {
            licenseFeedback.innerHTML = '<span class="log-error">Silakan masukkan License Key terlebih dahulu.</span>';
            return;
        }

        try {
            const res = await fetch(`${API_BASE}/license/activate`, {
                method: 'POST',
                headers: { 'Content-Type': 'application/json' },
                body: JSON.stringify({ license_key: key })
            });
            const data = await res.json();
            if (data.is_pro) {
                licenseFeedback.innerHTML = `<span class="log-success">✅ Berhasil! Lisensi Pro aktif untuk <strong>${escapeHtml(data.client_name)}</strong>.</span>`;
                fetchLicenseStatus();
                setTimeout(() => { licenseModal.style.display = 'none'; }, 1500);
            } else {
                licenseFeedback.innerHTML = '<span class="log-error">❌ Lisensi Key tidak valid atau signature tidak cocok.</span>';
            }
        } catch (err) {
            licenseFeedback.innerHTML = '<span class="log-error">❌ Gagal mengaktifkan lisensi.</span>';
        }
    });

    // Event Listeners
    btnSendChat.addEventListener('click', sendChatMessage);
    chatInput.addEventListener('keypress', (e) => {
        if (e.key === 'Enter') sendChatMessage();
    });

    btnAnalyzeLogs.addEventListener('click', analyzeLogs);

    btnRefresh.addEventListener('click', () => {
        appendLog('[SYSTEM] Refreshing telemetri...', 'info');
        fetchSystemStatus();
        fetchLicenseStatus();
    });

    btnFixNetwork.addEventListener('click', () => triggerAction('fix/network', 'Fix Network & DNS'));
    btnFixSpooler.addEventListener('click', () => triggerAction('fix/spooler', 'Fix Printer Spooler'));
    btnFixSfc.addEventListener('click', () => triggerAction('fix/sfc', 'System Repair (SFC)'));
    btnFixGpupdate.addEventListener('click', () => triggerAction('fix/gpupdate', 'Group Policy Update'));

    btnClearLog.addEventListener('click', () => {
        consoleOutput.innerHTML = '';
        appendLog('[SYSTEM] Logs cleared.', 'info');
    });

    // Initial Fetch & Interval Polling (every 3s)
    fetchSystemStatus();
    fetchLicenseStatus();
    setInterval(fetchSystemStatus, 3000);
});
