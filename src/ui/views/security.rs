use crate::ui::components::page_view::PageView;
use crate::ui::TranslationKey;
use gpui::*;
use gpui_component::{
    ActiveTheme, Disableable, Icon, StyledExt,
    button::{Button, ButtonCustomVariant, ButtonVariants},
    h_flex,
    switch::Switch,
    v_flex,
};

pub struct SecurityView;

impl SecurityView {
    pub fn build<V: 'static>(cx: &mut Context<V>) -> impl IntoElement {
        let (fg, muted_fg, border, card_bg) = {
            let theme = cx.theme();
            (
                theme.foreground,
                theme.muted_foreground,
                theme.border,
                theme.secondary,
            )
        };

        let destructive_red = rgb(0xef4444);
        let destructive_red_hover = rgb(0xdc2626);
        let destructive_red_active = rgb(0xb91c1c);
        let destructive_border = rgba(0xef44444d);
        let destructive_bg_muted = rgba(0xef44441a);

        let content = v_flex()
            .gap_6()
            .w_full()
            .child(
                v_flex()
                    .w_full()
                    .p_4()
                    .gap_2()
                    .border_1()
                    .border_color(destructive_border)
                    .bg(card_bg)
                    .rounded_md()
                    .child(
                        h_flex()
                            .gap_2()
                            .items_center()
                            .child(
                                Icon::default()
                                    .path("icons/triangle-alert.svg")
                                    .text_color(destructive_red),
                            )
                            .child(
                                div()
                                    .font_bold()
                                    .text_color(destructive_red)
                                    .child(crate::i18n::t(TranslationKey::FeatureUnstable)),
                            ),
                    )
                    .child(
                        div()
                            .text_sm()
                            .text_color(destructive_red)
                            .child(crate::i18n::t(TranslationKey::FeatureDisabledWarning)),
                    ),
            )
            .child(
                v_flex()
                    .w_full()
                    .border_1()
                    .border_color(destructive_border)
                    .bg(card_bg)
                    .rounded_xl()
                    .overflow_hidden()
                    .child(
                        div().p_6().child(
                            div()
                                .text_lg()
                                .font_bold()
                                .text_color(fg)
                                .child(crate::i18n::t(TranslationKey::LockSettings)),
                        ),
                    )
                    // Card Content
                    .child(
                        v_flex()
                            .px_6()
                            .pb_6()
                            .gap_6()
                            .child(
                                h_flex()
                                    .justify_between()
                                    .items_center()
                                    .child(
                                        v_flex()
                                            .gap_1()
                                            .child(
                                                div()
                                                    .text_sm()
                                                    .font_medium()
                                                    .child(crate::i18n::t(TranslationKey::EnableSecureBootLabel)),
                                            )
                                            .child(
                                                div().text_xs().text_color(muted_fg).child(
                                                    crate::i18n::t(TranslationKey::EnableSecureBootDesc),
                                                ),
                                            ),
                                    )
                                    .child(
                                        Switch::new("secure-boot-switch")
                                            .checked(false)
                                            .disabled(true),
                                    ),
                            )
                            .child(
                                // Secure Lock Row (Disabled)
                                h_flex()
                                    .justify_between()
                                    .items_center()
                                    .child(
                                        v_flex()
                                            .gap_1()
                                            .child(
                                                div().text_sm().font_medium().child(crate::i18n::t(TranslationKey::SecureLockLabel)),
                                            )
                                            .child(div().text_xs().text_color(muted_fg).child(
                                                crate::i18n::t(TranslationKey::SecureLockDesc),
                                            )),
                                    )
                                    .child(
                                        Switch::new("secure-lock-switch")
                                            .checked(false)
                                            .disabled(true),
                                    ),
                            )
                            .child(div().h_px().bg(border))
                            .child(
                                h_flex()
                                    .items_center()
                                    .gap_4()
                                    .p_4()
                                    .rounded_md()
                                    .bg(destructive_bg_muted)
                                    .border_1()
                                    .border_color(destructive_border)
                                    .child(
                                        Switch::new("confirm-switch").checked(false).disabled(true),
                                    )
                                    .child(
                                        div()
                                            .font_medium()
                                            .text_color(destructive_red)
                                            .child(crate::i18n::t(TranslationKey::UnderstandRisks)),
                                    ),
                            ),
                    )
                    // Card Footer
                    .child(
                        div()
                            .border_t_1()
                            .border_color(border)
                            .bg(gpui::rgba(0x00000033))
                            .px_6()
                            .py_4()
                            .flex()
                            .justify_end()
                            .child(
                                Button::new("lock-device-btn")
                                    .custom(
                                        ButtonCustomVariant::new(cx)
                                            .color(destructive_red.into())
                                            .hover(destructive_red_hover.into())
                                            .active(destructive_red_active.into()),
                                    )
                                    .disabled(true)
                                    .child(
                                        h_flex()
                                            .gap_2()
                                            .items_center()
                                            .child(Icon::default().path("icons/lock.svg").size_4())
                                            .child(crate::i18n::t(TranslationKey::PermanentlyLockDevice)),
                                    ),
                            ),
                    ),
            );

        PageView::build(
            crate::i18n::t(TranslationKey::SecurityTitle),
            crate::i18n::t(TranslationKey::SecurityDescription),
            content,
            cx.theme(),
        )
    }
}
