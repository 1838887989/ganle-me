import sharp from 'sharp';
import path from 'path';
import { fileURLToPath } from 'url';
import fs from 'fs';
import pngToIco from 'png-to-ico';

const __filename = fileURLToPath(import.meta.url);
const __dirname = path.dirname(__filename);

const OUTPUT_PNG = path.join(__dirname, '../build/icon.png');
const OUTPUT_ICO = path.join(__dirname, '../build/icon.ico');
const SIZE = 1024;

async function generateLogo() {
    console.log('生成「干了么」图标...');

    // 圆角矩形背景 - 渐变绿色（代表完成、done）
    const backgroundSvg = `
    <svg width="${SIZE}" height="${SIZE}" viewBox="0 0 1024 1024">
        <defs>
            <linearGradient id="grad" x1="0%" y1="0%" x2="100%" y2="100%">
                <stop offset="0%" style="stop-color:#10B981;stop-opacity:1" />
                <stop offset="100%" style="stop-color:#059669;stop-opacity:1" />
            </linearGradient>
        </defs>
        <rect x="0" y="0" width="1024" height="1024" rx="220" ry="220" fill="url(#grad)" />
    </svg>`;

    // "干" 字 - 白色粗体
    const textSvg = `
    <svg width="${SIZE}" height="${SIZE}" viewBox="0 0 1024 1024">
        <text 
            x="50%" 
            y="54%" 
            text-anchor="middle" 
            dominant-baseline="middle" 
            font-family="Microsoft YaHei, PingFang SC, Hiragino Sans GB, sans-serif" 
            font-size="580" 
            font-weight="900" 
            fill="white"
        >干</text>
    </svg>`;

    try {
        // 确保 build 目录存在
        const buildDir = path.dirname(OUTPUT_PNG);
        if (!fs.existsSync(buildDir)) {
            fs.mkdirSync(buildDir, { recursive: true });
        }

        // 生成 PNG
        const pngBuffer = await sharp({
            create: {
                width: SIZE,
                height: SIZE,
                channels: 4,
                background: { r: 0, g: 0, b: 0, alpha: 0 }
            }
        })
        .composite([
            { input: Buffer.from(backgroundSvg), blend: 'over' },
            { input: Buffer.from(textSvg), blend: 'over' }
        ])
        .png()
        .toBuffer();

        fs.writeFileSync(OUTPUT_PNG, pngBuffer);
        console.log(`✅ 生成 ${OUTPUT_PNG}`);

        // 生成 ICO
        console.log('生成 ICO...');
        const icoBuffer = await pngToIco(pngBuffer);
        fs.writeFileSync(OUTPUT_ICO, icoBuffer);
        console.log(`✅ 生成 ${OUTPUT_ICO}`);

    } catch (err) {
        console.error('❌ 生成失败:', err);
    }
}

generateLogo();
