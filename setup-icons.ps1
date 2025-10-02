# EV Charger Simulator - Icon Setup Script
# This script downloads and sets up the application icons

Write-Host "========================================" -ForegroundColor Cyan
Write-Host "EV Charger Simulator - Icon Setup" -ForegroundColor Cyan
Write-Host "========================================" -ForegroundColor Cyan
Write-Host ""

$iconsDir = Join-Path $PSScriptRoot "src-tauri\icons"

# Create icons directory if it doesn't exist
if (!(Test-Path $iconsDir)) {
    Write-Host "Creating icons directory..." -ForegroundColor Yellow
    New-Item -ItemType Directory -Path $iconsDir -Force | Out-Null
}

# Download a simple charging station icon from a reliable source
$iconUrl = "https://raw.githubusercontent.com/tauri-apps/tauri/dev/tooling/cli/templates/app/app-icon.png"
$tempPngPath = Join-Path $iconsDir "temp-icon.png"

Write-Host "Downloading icon template..." -ForegroundColor Yellow
try {
    Invoke-WebRequest -Uri $iconUrl -OutFile $tempPngPath -ErrorAction Stop
    Write-Host "✓ Icon downloaded successfully" -ForegroundColor Green
} catch {
    Write-Host "✗ Failed to download icon: $_" -ForegroundColor Red
    Write-Host ""
    Write-Host "Please manually create an icon file following these steps:" -ForegroundColor Yellow
    Write-Host "1. Create a 1024x1024 PNG image" -ForegroundColor White
    Write-Host "2. Save it to: $iconsDir\app-icon.png" -ForegroundColor White
    Write-Host "3. Run: npm run tauri icon src-tauri\icons\app-icon.png" -ForegroundColor White
    exit 1
}

# Check if Tauri CLI is installed
Write-Host ""
Write-Host "Checking for Tauri CLI..." -ForegroundColor Yellow

$tauriInstalled = $false
try {
    $null = Get-Command tauri -ErrorAction Stop
    $tauriInstalled = $true
    Write-Host "✓ Tauri CLI is installed" -ForegroundColor Green
} catch {
    Write-Host "✗ Tauri CLI not found" -ForegroundColor Red
}

if (!$tauriInstalled) {
    Write-Host ""
    Write-Host "Installing Tauri CLI globally..." -ForegroundColor Yellow
    try {
        npm install -g @tauri-apps/cli --loglevel error
        Write-Host "✓ Tauri CLI installed successfully" -ForegroundColor Green
        $tauriInstalled = $true
    } catch {
        Write-Host "✗ Failed to install Tauri CLI: $_" -ForegroundColor Red
        Write-Host ""
        Write-Host "You can manually install it by running:" -ForegroundColor Yellow
        Write-Host "npm install -g @tauri-apps/cli" -ForegroundColor White
    }
}

# Generate icons using Tauri CLI
if ($tauriInstalled) {
    Write-Host ""
    Write-Host "Generating platform-specific icons..." -ForegroundColor Yellow
    
    try {
        Set-Location $PSScriptRoot
        npm run tauri icon $tempPngPath 2>&1 | Out-Null
        
        if (Test-Path (Join-Path $iconsDir "icon.ico")) {
            Write-Host "✓ Icons generated successfully" -ForegroundColor Green
            
            # Clean up temp file
            Remove-Item $tempPngPath -Force -ErrorAction SilentlyContinue
            
            Write-Host ""
            Write-Host "========================================" -ForegroundColor Green
            Write-Host "Icon setup completed successfully!" -ForegroundColor Green
            Write-Host "========================================" -ForegroundColor Green
            Write-Host ""
            Write-Host "You can now build the project with:" -ForegroundColor Cyan
            Write-Host "  npm run tauri build" -ForegroundColor White
            Write-Host ""
        } else {
            Write-Host "✗ Icon generation failed" -ForegroundColor Red
            Write-Host ""
            Write-Host "Manual steps:" -ForegroundColor Yellow
            Write-Host "1. Rename $tempPngPath to app-icon.png" -ForegroundColor White
            Write-Host "2. Run: npm run tauri icon src-tauri\icons\app-icon.png" -ForegroundColor White
        }
    } catch {
        Write-Host "✗ Error generating icons: $_" -ForegroundColor Red
    }
} else {
    Write-Host ""
    Write-Host "========================================" -ForegroundColor Yellow
    Write-Host "Manual icon generation required" -ForegroundColor Yellow
    Write-Host "========================================" -ForegroundColor Yellow
    Write-Host ""
    Write-Host "Steps:" -ForegroundColor Cyan
    Write-Host "1. Install Tauri CLI: npm install -g @tauri-apps/cli" -ForegroundColor White
    Write-Host "2. Generate icons: npm run tauri icon $tempPngPath" -ForegroundColor White
    Write-Host ""
}
