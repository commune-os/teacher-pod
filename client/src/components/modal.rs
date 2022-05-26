use dioxus::{prelude::*, web::use_eval};
use dioxus_heroicons::{Icon, solid::Shape};

use crate::PLAYER_STATUS;

pub fn PlayBox(cx: Scope) -> Element {

    let status = use_atom_ref(&cx, PLAYER_STATUS);
    let player_hidden = if !status.read().display {
        "hidden"
    } else { "hidden sm:block" };
    let icon_hidden = if status.read().display {
        "hidden"
    } else { "hidden sm:block" };

    let eval_script = use_eval::<&str>(&cx);

    cx.render(rsx! {
        div {
            class: "{icon_hidden} fixed bottom-12 left-2 rounded-full w-10 h-10 
            bg-white dark:bg-gray-900 hover:bg-black dark:hover:bg-white",
            button {
                class: "justify-center w-full h-full text-black dark:text-white hover:text-white dark:hover:text-black",
                onclick: move |_| {
                    status.write().display = true;
                },
                Icon {
                    class: "h-full w-full",
                    icon: Shape::Play,
                }
            }
        }
        div {
            class: "{player_hidden} fixed bottom-8 left-0 w-1/3 h-20 rounded
            bg-white dark:bg-gray-900 shadow-2xl
            px-2 py-2 z-40
            ",
            div {
                class: "flex h-full gap-2",
                div {
                    class: "flex-initial w-16",
                    img {
                        class: "h-full rounded",
                        src: "https://picsum.photos/seed/2/200/200",
                    }
                }
                div {
                    class: "grow relative",
                    div {
                        span {
                            class: "text-black dark:text-white",
                            "66. About Us - TeacherPod"
                        }
                        span {
                            class: "absolute right-0",
                            div {
                                class: "flex items-center justify-center",
                                div {
                                    class: "inline-flex",
                                    role: "group",
                                    button {
                                        class: "rounded-full inline-block px-1 py-1 text-black dark:text-white font-medium text-xs leading-tight hover:bg-gray-800 hover:text-white transition duration-150 ease-in-out",
                                        r#type: "button",
                                        Icon {
                                            icon: Shape::Star
                                        }   
                                    }
                                    button {
                                        class: "rounded-full inline-block px-1 py-1 text-black dark:text-white font-medium text-xs leading-tight hover:bg-gray-800 hover:text-white transition duration-150 ease-in-out",
                                        r#type: "button",
                                        onclick: move |_| {
                                            status.write().display = false;
                                        },
                                        Icon {
                                            icon: Shape::MinusCircle
                                        }   
                                    }
                                }
                            }
                        }
                    }
                    div {
                        class: "absolute bottom-0 w-full",
                        // input {
                        //     class: "w-full h-1",
                        //     r#type: "range",
                        //     min: "0",
                        //     max: "60",
                        //     step: "1",
                        //     value: "0"
                        // }
                        audio {
                            class: "w-full h-8",
                            controls: "controls",
                            "controlsList": "nodownload",
                            autoplay: "true",
                            "oncontextmenu": "return false",
                            onpause: |e| {
                                log::info!("{:?}", e);
                                eval_script("alert(1)");
                            },
                            source {
                                src: "/assets/test.mp3",
                                "type": "audio/mp3"
                            }
                        }
                    }
                }
            }
        }
    })
}