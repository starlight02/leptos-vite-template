import 'mdui/mdui.css';

import 'mdui/components/button';
import 'mdui/components/button-icon';
import 'mdui/components/card';
import 'mdui/components/checkbox';
import 'mdui/components/icon';

import { alert } from 'mdui/functions/alert';
import { confirm } from 'mdui/functions/confirm';
import { dialog } from 'mdui/functions/dialog';
import { snackbar } from 'mdui/functions/snackbar';


export const mdui = {
  alert,
  confirm,
  snackbar: (options: any) => new Promise((resolve, reject) => {
    try {
      resolve(snackbar(options))
    } catch (e) {
      reject(e)
    }
  })
};

// 导出到全局以便 Rust 绑定使用
if (typeof window !== 'undefined') {
  (window as any).mdui = mdui;
  console.log('📦 支持的功能:', Object.keys(mdui));
}
