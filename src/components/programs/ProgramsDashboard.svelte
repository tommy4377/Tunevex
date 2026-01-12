<script lang="ts">
    import { invoke } from "@tauri-apps/api/core";
    import { fade } from "svelte/transition";

    interface Program {
        id: string;
        name: string;
        description: string;
        category: string;
        installed?: boolean; // We might verify this
    }

    // List sourced from official WinUtil applications.json
    const allPrograms: Program[] = [
        // Browsers
        {
            id: "Google.Chrome",
            name: "Google Chrome",
            description: "Fast, secure web browser.",
            category: "Browsers",
        },
        {
            id: "Mozilla.Firefox",
            name: "Mozilla Firefox",
            description: "Privacy-focused web browser.",
            category: "Browsers",
        },
        {
            id: "Brave.Brave",
            name: "Brave Browser",
            description: "Secure, fast, private web browser.",
            category: "Browsers",
        },
        {
            id: "Microsoft.Edge",
            name: "Microsoft Edge",
            description: "AI-powered browser by Microsoft.",
            category: "Browsers",
        },
        {
            id: "Hibbiki.Chromium",
            name: "Chromium",
            description: "Open-source web browser foundation.",
            category: "Browsers",
        },
        {
            id: "Waterfox.Waterfox",
            name: "Waterfox",
            description: "Privacy-focused Firefox fork.",
            category: "Browsers",
        },
        {
            id: "KDE.Falkon",
            name: "Falkon",
            description: "Lightweight and fast web browser.",
            category: "Browsers",
        },
        {
            id: "Ablaze.Floorp",
            name: "Floorp",
            description: "Simple and fast browser.",
            category: "Browsers",
        },
        {
            id: "Mozilla.Firefox.ESR",
            name: "Firefox ESR",
            description: "Extended Support Release of Firefox.",
            category: "Browsers",
        },

        // Communication
        {
            id: "Discord.Discord",
            name: "Discord",
            description: "Voice, video and text chat.",
            category: "Communication",
        },
        {
            id: "Telegram.TelegramDesktop",
            name: "Telegram Desktop",
            description: "Fast and secure messaging.",
            category: "Communication",
        },
        {
            id: "Zoom.Zoom",
            name: "Zoom",
            description: "Video conferencing.",
            category: "Communication",
        },
        {
            id: "WhatsApp.WhatsApp",
            name: "WhatsApp",
            description: "Messaging and calling.",
            category: "Communication",
        },
        {
            id: "Ferdium.Ferdium",
            name: "Ferdium",
            description: "Combine all messaging services into one.",
            category: "Communication",
        },
        {
            id: "ChatterinoTeam.Chatterino",
            name: "Chatterino",
            description: "Chat client for Twitch.",
            category: "Communication",
        },
        {
            id: "Beeper.Beeper",
            name: "Beeper",
            description: "All your chats in one app.",
            category: "Communication",
        },

        // Development
        {
            id: "Microsoft.VisualStudioCode",
            name: "VS Code",
            description: "Code editor redefined.",
            category: "Development",
        },
        {
            id: "VSCodium.VSCodium",
            name: "VSCodium",
            description: "Free/Libre Open Source Software Binaries of VS Code.",
            category: "Development",
        },
        {
            id: "Git.Git",
            name: "Git",
            description: "Distributed version control.",
            category: "Development",
        },
        {
            id: "GitHub.GitHubDesktop",
            name: "GitHub Desktop",
            description: "Visual Git client.",
            category: "Development",
        },
        {
            id: "GitHub.cli",
            name: "GitHub CLI",
            description: "Command line for GitHub.",
            category: "Development",
        },
        {
            id: "GitExtensionsTeam.GitExtensions",
            name: "Git Extensions",
            description: "Graphical user interface for Git.",
            category: "Development",
        },
        {
            id: "Notepad++.Notepad++",
            name: "Notepad++",
            description: "Free source code editor.",
            category: "Development",
        },
        {
            id: "Python.Python.3",
            name: "Python 3",
            description: "Programming language.",
            category: "Development",
        },
        {
            id: "OpenJS.NodeJS.LTS",
            name: "Node.js (LTS)",
            description: "JavaScript runtime.",
            category: "Development",
        },
        {
            id: "Anaconda.Anaconda3",
            name: "Anaconda",
            description: "Python/R distribution for scientific computing.",
            category: "Development",
        },
        {
            id: "Kitware.CMake",
            name: "CMake",
            description: "Tools to build, test and package software.",
            category: "Development",
        },
        {
            id: "Docker.DockerDesktop",
            name: "Docker Desktop",
            description: "Containerized application development.",
            category: "Development",
        },
        {
            id: "chrisant996.Clink",
            name: "Clink",
            description: "Bash-compatible CLI enhancement.",
            category: "Development",
        },
        {
            id: "wez.wezterm",
            name: "WezTerm",
            description: "GPU-accelerated terminal emulator.",
            category: "Development",
        },
        {
            id: "Alacritty.Alacritty",
            name: "Alacritty",
            description: "Fast, cross-platform, OpenGL terminal emulator.",
            category: "Development",
        },

        // Utilities
        {
            id: "7zip.7zip",
            name: "7-Zip",
            description: "File archiver.",
            category: "Utilities",
        },
        {
            id: "M2Team.NanaZip",
            name: "NanaZip",
            description: "7-Zip fork for Windows 11.",
            category: "Utilities",
        },
        {
            id: "Microsoft.PowerToys",
            name: "PowerToys",
            description: "System utilities for power users.",
            category: "Utilities",
        },
        {
            id: "BleachBit.BleachBit",
            name: "BleachBit",
            description: "Disk space cleaner.",
            category: "Utilities",
        },
        {
            id: "AntibodySoftware.WizTree",
            name: "WizTree",
            description: "Disk space analysis.",
            category: "Utilities",
        },
        {
            id: "voidtools.Everything",
            name: "Everything",
            description: "Locate files and folders by name instantly.",
            category: "Utilities",
        },
        {
            id: "Flow-Launcher.Flow-Launcher",
            name: "Flow Launcher",
            description: "Quick file search and app launcher.",
            category: "Utilities",
        },
        {
            id: "CPUID.CPU-Z",
            name: "CPU-Z",
            description: "System monitoring and diagnostic.",
            category: "Utilities",
        },
        {
            id: "CrystalDewWorld.CrystalDiskInfo",
            name: "CrystalDiskInfo",
            description: "HDD/SSD health monitoring.",
            category: "Utilities",
        },
        {
            id: "CrystalDewWorld.CrystalDiskMark",
            name: "CrystalDiskMark",
            description: "Disk benchmarking.",
            category: "Utilities",
        },
        {
            id: "AgileBits.1Password",
            name: "1Password",
            description: "Password manager.",
            category: "Utilities",
        },
        {
            id: "Bitwarden.Bitwarden",
            name: "Bitwarden",
            description: "Open source password manager.",
            category: "Utilities",
        },
        {
            id: "AnyDesk.AnyDesk",
            name: "AnyDesk",
            description: "Remote desktop software.",
            category: "Utilities",
        },
        {
            id: "Microsoft.Sysinternals.Autoruns",
            name: "Autoruns",
            description: "See what programs are configured to startup.",
            category: "Utilities",
        },
        {
            id: "Klocman.BulkCrapUninstaller",
            name: "Bulk Crap Uninstaller",
            description: "Remove large amounts of unwanted software.",
            category: "Utilities",
        },
        {
            id: "TGRMNSoftware.BulkRenameUtility",
            name: "Bulk Rename Utility",
            description: "Rename multiple files at once.",
            category: "Utilities",
        },
        {
            id: "HulubuluSoftware.AdvancedRenamer",
            name: "Advanced Renamer",
            description: "Batch rename files and folders.",
            category: "Utilities",
        },
        {
            id: "code52.Carnac",
            name: "Carnac",
            description: "Keystroke visualizer.",
            category: "Utilities",
        },
        {
            id: "hluk.CopyQ",
            name: "CopyQ",
            description: "Clipboard manager with advanced features.",
            category: "Utilities",
        },
        {
            id: "Ditto.Ditto",
            name: "Ditto",
            description: "Extension to the standard Windows clipboard.",
            category: "Utilities",
        },
        {
            id: "Duplicati.Duplicati",
            name: "Duplicati",
            description: "Free backup software.",
            category: "Utilities",
        },
        {
            id: "Espanso.Espanso",
            name: "Espanso",
            description: "Cross-platform text expander.",
            category: "Utilities",
        },
        {
            id: "Fastfetch-cli.Fastfetch",
            name: "Fastfetch",
            description: "Neofetch-like tool for system info.",
            category: "Utilities",
        },
        {
            id: "AdrienAllard.FileConverter",
            name: "File Converter",
            description: "Context menu file conversion tool.",
            category: "Utilities",
        },
        {
            id: "flux.flux",
            name: "f.lux",
            description: "Adjusts screen color temperature.",
            category: "Utilities",
        },
        {
            id: "junegunn.fzf",
            name: "Fzf",
            description: "Command-line fuzzy finder.",
            category: "Utilities",
        },
        {
            id: "WinMerge.WinMerge",
            name: "WinMerge",
            description: "Differencing and merging tool.",
            category: "Utilities",
        },
        {
            id: "MartiCliment.UniGetUI",
            name: "UniGetUI",
            description: "GUI for Winget/Chocolatey.",
            category: "Utilities",
        },
        {
            id: "Wagnardsoft.DisplayDriverUninstaller",
            name: "DDU",
            description: "Display Driver Uninstaller.",
            category: "Utilities",
        },
        {
            id: "Microsoft.WindowsPCHealthCheck",
            name: "PC Health Check",
            description: "Check if PC meets Win11 requirements.",
            category: "Utilities",
        },

        // Multimedia
        {
            id: "VideoLAN.VLC",
            name: "VLC Media Player",
            description: "Open source multimedia player.",
            category: "Multimedia",
        },
        {
            id: "Spotify.Spotify",
            name: "Spotify",
            description: "Digital music service.",
            category: "Multimedia",
        },
        {
            id: "OBSProject.OBSStudio",
            name: "OBS Studio",
            description: "Live streaming software.",
            category: "Multimedia",
        },
        {
            id: "PaintDotNet.PaintDotNet",
            name: "Paint.net",
            description: "Image and photo editing.",
            category: "Multimedia",
        },
        {
            id: "GIMP.GIMP.3",
            name: "GIMP",
            description: "GNU Image Manipulation Program.",
            category: "Multimedia",
        },
        {
            id: "BlenderFoundation.Blender",
            name: "Blender",
            description: "3D creation suite.",
            category: "Multimedia",
        },
        {
            id: "Audacity.Audacity",
            name: "Audacity",
            description: "Audio editor and recorder.",
            category: "Multimedia",
        },
        {
            id: "AIMP.AIMP",
            name: "AIMP",
            description: "Music player.",
            category: "Multimedia",
        },
        {
            id: "Clementine.Clementine",
            name: "Clementine",
            description: "Music player and library organizer.",
            category: "Multimedia",
        },
        {
            id: "PeterPawlowski.foobar2000",
            name: "foobar2000",
            description: "Advanced audio player.",
            category: "Multimedia",
        },
        {
            id: "Gyan.FFmpeg",
            name: "FFmpeg",
            description:
                "Complete solution to record, convert and stream audio/video.",
            category: "Multimedia",
        },
        {
            id: "Flameshot.Flameshot",
            name: "Flameshot",
            description: "Powerful screenshot software.",
            category: "Multimedia",
        },
        {
            id: "Skillbrains.Lightshot",
            name: "Lightshot",
            description: "Lightweight screenshot tool.",
            category: "Multimedia",
        },
        {
            id: "KDE.digikam",
            name: "digiKam",
            description: "Professional photo management.",
            category: "Multimedia",
        },
        {
            id: "FireAlpaca.FireAlpaca",
            name: "FireAlpaca",
            description: "Digital painting software.",
            category: "Multimedia",
        },
        {
            id: "FxSound.FxSound",
            name: "FxSound",
            description: "Boost sound quality and volume.",
            category: "Multimedia",
        },
        {
            id: "VB-Audio.Voicemeeter",
            name: "Voicemeeter",
            description: "Virtual Audio Mixer.",
            category: "Multimedia",
        },
        {
            id: "VB-Audio.Voicemeeter.Potato",
            name: "Voicemeeter Potato",
            description: "Ultimate Audio Mixer.",
            category: "Multimedia",
        },
        {
            id: "File-New-Project.EarTrumpet",
            name: "EarTrumpet",
            description: "Advanced volume control app.",
            category: "Multimedia",
        },

        // Games
        {
            id: "EpicGames.EpicGamesLauncher",
            name: "Epic Games",
            description: "Store and launcher.",
            category: "Games",
        },
        {
            id: "ElectronicArts.EADesktop",
            name: "EA App",
            description: "EA Games launcher.",
            category: "Games",
        },
        {
            id: "Nvidia.GeForceNow",
            name: "GeForce NOW",
            description: "Cloud gaming service.",
            category: "Games",
        },
        {
            id: "Cemu.Cemu",
            name: "Cemu",
            description: "Wii U emulator.",
            category: "Games",
        },
        {
            id: "CloneHeroTeam.CloneHero",
            name: "Clone Hero",
            description: "Rhythm game.",
            category: "Games",
        },
        {
            id: "Emulationstation.Emulationstation",
            name: "EmulationStation",
            description: "Emulator front-end.",
            category: "Games",
        },
        {
            id: "VirtualDesktop.Streamer",
            name: "Virtual Desktop",
            description: "Stream PC to VR.",
            category: "Games",
        },

        // Document
        {
            id: "Adobe.Acrobat.Reader.64-bit",
            name: "Adobe Reader",
            description: "View and print PDF files.",
            category: "Document",
        },
        {
            id: "Foxit.FoxitReader",
            name: "Foxit Reader",
            description: "Fast, feature-rich PDF reader.",
            category: "Document",
        },
        {
            id: "Foxit.PhantomPDF",
            name: "Foxit Editor",
            description: "PDF Editor.",
            category: "Document",
        },
        {
            id: "ToEverything.AFFiNE",
            name: "AFFiNE",
            description: "Notion alternative.",
            category: "Document",
        },
        {
            id: "Anki.Anki",
            name: "Anki",
            description: "Flashcards.",
            category: "Document",
        },
        {
            id: "calibre.calibre",
            name: "Calibre",
            description: "E-book management.",
            category: "Document",
        },
    ];

    let activeTab = "Browsers";
    const categories = [
        "Browsers",
        "Communication",
        "Development",
        "Utilities",
        "Multimedia",
        "Games",
        "Document",
    ];

    let processingMap: Record<string, string> = {}; // id -> status string

    $: displayPrograms = allPrograms.filter((p) => p.category === activeTab);

    async function install(id: string) {
        if (processingMap[id]) return;
        processingMap[id] = "Installing...";
        try {
            await invoke("install_package", { id });
            processingMap[id] = "Installed ✅";
        } catch (e) {
            console.error(e);
            processingMap[id] = "Error ❌";
        } finally {
            // keep status for a bit
            setTimeout(() => {
                if (processingMap[id] === "Installed ✅")
                    delete processingMap[id];
            }, 3000);
        }
    }

    async function uninstall(id: string) {
        if (processingMap[id]) return;
        processingMap[id] = "Uninstalling...";
        try {
            await invoke("uninstall_package", { id });
            processingMap[id] = "Uninstalled 🗑️";
        } catch (e) {
            console.error(e);
            processingMap[id] = "Error ❌";
        } finally {
            setTimeout(() => {
                if (processingMap[id] === "Uninstalled 🗑️")
                    delete processingMap[id];
            }, 3000);
        }
    }
</script>

<div class="programs-container" in:fade>
    <div class="tabs">
        {#each categories as cat}
            <button
                class:active={activeTab === cat}
                on:click={() => (activeTab = cat)}
            >
                {cat}
            </button>
        {/each}
    </div>

    <div class="program-grid">
        {#each displayPrograms as prog}
            <div class="card">
                <div class="card-content">
                    <h3>{prog.name}</h3>
                    <p class="desc">{prog.description}</p>
                    <code class="pkg-id">{prog.id}</code>
                </div>
                <div class="card-actions">
                    {#if processingMap[prog.id]}
                        <div class="status">{processingMap[prog.id]}</div>
                    {:else}
                        <button
                            class="action-btn install"
                            on:click={() => install(prog.id)}>Install</button
                        >
                        <button
                            class="action-btn uninstall"
                            on:click={() => uninstall(prog.id)}
                            >Uninstall</button
                        >
                    {/if}
                </div>
            </div>
        {/each}
    </div>
</div>

<style>
    .programs-container {
        height: 100%;
        display: flex;
        flex-direction: column;
        overflow: hidden;
    }

    .tabs {
        display: flex;
        padding: 16px 24px;
        gap: 8px;
        border-bottom: 1px solid var(--border-color);
        overflow-x: auto;
    }

    .tabs button {
        background: transparent;
        border: 1px solid transparent;
        color: var(--text-muted);
        padding: 8px 16px;
        border-radius: 20px;
        cursor: pointer;
        font-size: 14px;
        font-weight: 500;
        white-space: nowrap;
        transition: all 0.2s;
    }

    .tabs button:hover {
        background: rgba(255, 255, 255, 0.05);
        color: var(--text-color);
    }

    .tabs button.active {
        background: var(--accent-color);
        color: white;
    }

    .program-grid {
        flex: 1;
        overflow-y: auto;
        padding: 24px;
        display: grid;
        grid-template-columns: repeat(auto-fill, minmax(220px, 1fr));
        gap: 20px;
    }

    .card {
        background: rgba(255, 255, 255, 0.03);
        border: 1px solid var(--border-color);
        border-radius: var(--radius);
        padding: 16px;
        display: flex;
        flex-direction: column;
        justify-content: space-between;
        gap: 12px;
    }

    .card-content h3 {
        margin: 0 0 4px 0;
        font-size: 16px;
        color: var(--text-color);
    }

    .desc {
        font-size: 13px;
        color: var(--text-muted);
        margin: 0 0 8px 0;
        line-height: 1.4;
    }

    .pkg-id {
        font-size: 11px;
        color: var(--text-muted);
        background: rgba(0, 0, 0, 0.2);
        padding: 2px 4px;
        border-radius: 4px;
        font-family: monospace;
    }

    .card-actions {
        display: flex;
        gap: 8px;
        margin-top: auto;
    }

    .action-btn {
        flex: 1;
        border: none;
        padding: 6px;
        border-radius: 4px;
        font-size: 12px;
        font-weight: 500;
        cursor: pointer;
        transition: opacity 0.2s;
    }
    .action-btn.install {
        background: #10b981;
        color: white;
    }
    .action-btn.uninstall {
        background: rgba(255, 255, 255, 0.1);
        color: var(--text-color);
    }
    .action-btn:hover {
        opacity: 0.9;
    }

    .status {
        width: 100%;
        text-align: center;
        font-size: 12px;
        font-weight: 500;
        color: var(--accent-color);
        padding: 6px;
    }
</style>
