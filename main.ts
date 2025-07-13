// 🎉 使用虚拟模块自动加载 WASM
import 'virtual:wasm-init';

// 🎨 导入样式文件
import './src/assets/styles.css';

console.log('🚀 Main.ts loaded - WASM auto-import configured!');
console.log('🎨 Styles loaded');

// 您可以在这里添加其他初始化代码
// 例如：全局事件监听器等

// 如果您需要手动控制 WASM 初始化，可以这样做：
// import { initWasm } from 'virtual:wasm-init';
// await initWasm();
