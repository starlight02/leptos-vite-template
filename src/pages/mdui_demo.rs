use leptos::prelude::*;

use crate::components::mdui_components::*;
// 3. 完整的演示页面
#[component]
pub fn MduiDemo() -> impl IntoView {
    // 使用 RwSignal 进行状态管理
    let count = RwSignal::new(0);
    let text_value = RwSignal::new(String::new());
    let dialog_open = RwSignal::new(false);
    let chip_selected = RwSignal::new(false);
    let snackbar_open = RwSignal::new(false);

    view! {
        <div class="mdui-demo-container">
            <h2>"MDUI + Leptos 0.8.2 演示"</h2>

            // 按钮组
            <div class="button-group">
                <MduiButton
                    variant="filled".to_string()
                    on_click=Box::new(move || {
                        count.update(|n| *n += 1);
                    })
                >
                    "计数: "
                    {move || count.get()}
                </MduiButton>

                <MduiButton
                    variant="outlined".to_string()
                    icon="refresh".to_string()
                    on_click=Box::new(move || {
                        count.set(0);
                    })
                >
                    "重置"
                </MduiButton>
            </div>

            // 输入框
            <div class="input-group">
                <MduiTextField
                    label="输入文本".to_string()
                    placeholder="请输入内容...".to_string()
                    value=text_value
                />
                <p>"输入内容: " {move || text_value.get()}</p>
            </div>

            // 芯片组件
            <div class="chip-group">
                <MduiChip variant="filter".to_string() selectable=true selected=chip_selected>
                    "可选择的标签"
                </MduiChip>
                <p>
                    "标签状态: "
                    {move || if chip_selected.get() { "已选择" } else { "未选择" }}
                </p>
            </div>

            // 卡片
            <MduiCard variant="elevated".to_string() clickable=true class="demo-card".to_string()>
                <div>
                    <h3>"信息卡片"1</h3>
                </div>
                <div>
                    <p>"这是一个使用 MDUI 和 Leptos 0.8.2 构建的卡片。"</p>
                    <p>"当前计数: " {move || count.get()}</p>
                </div>
                <div>
                    <MduiButton
                        variant="text".to_string()
                        on_click=Box::new(move || {
                            dialog_open.set(true);
                        })
                    >
                        "打开对话框"
                    </MduiButton>

                    <MduiButton
                        variant="text".to_string()
                        on_click=Box::new(move || {
                            snackbar_open.set(true);
                        })
                    >
                        "显示通知"
                    </MduiButton>
                </div>
            </MduiCard>

            // 对话框
            <MduiDialog open=dialog_open headline="确认对话框".to_string()>
                <div slot="content">
                    <p>"这是一个使用 Leptos 0.8.2 构建的对话框。"</p>
                    <p>"计数值: " {move || count.get()}</p>
                </div>
                <div slot="action">
                    <MduiButton
                        variant="text".to_string()
                        on_click=Box::new(move || {
                            dialog_open.set(false);
                        })
                    >
                        "关闭"
                    </MduiButton>
                </div>
            </MduiDialog>

            // 通知条
            <MduiSnackbar
                message="操作成功完成！".to_string()
                action="撤销".to_string()
                closeable=true
                open=snackbar_open
            />
        </div>
    }
}