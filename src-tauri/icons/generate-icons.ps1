# 生成简单的图标文件
# 由于我们没有实际的图标文件，创建一个简单的 SVG 并转换

# 创建 32x32 PNG
$svg32 = @"
<svg xmlns="http://www.w3.org/2000/svg" width="32" height="32" viewBox="0 0 32 32">
  <rect width="32" height="32" fill="#3b82f6" rx="6"/>
  <text x="16" y="22" font-family="Arial" font-size="16" font-weight="bold" fill="white" text-anchor="middle">S</text>
</svg>
"@

# 创建 128x128 PNG
$svg128 = @"
<svg xmlns="http://www.w3.org/2000/svg" width="128" height="128" viewBox="0 0 128 128">
  <rect width="128" height="128" fill="#3b82f6" rx="24"/>
  <text x="64" y="88" font-family="Arial" font-size="72" font-weight="bold" fill="white" text-anchor="middle">S</text>
</svg>
"@

# 保存 SVG 文件
$svg32 | Out-File -FilePath "32x32.svg" -Encoding UTF8
$svg128 | Out-File -FilePath "128x128.svg" -Encoding UTF8

Write-Host "SVG icons created. Please convert them to PNG format using an image editor or online tool."
Write-Host "Or manually create the following files:"
Write-Host "- 32x32.png"
Write-Host "- 128x128.png"
Write-Host "- 128x128@2x.png (256x256)"
Write-Host "- icon.ico"
Write-Host "- icon.icns"
