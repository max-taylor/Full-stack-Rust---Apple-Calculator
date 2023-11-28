use yew::{classes, Classes};

pub fn get_button_styles(extra_classes: Classes) -> Classes {
    classes!(vec![
        classes!(
            "flex",
            "flex-1",
            "items-center",
            "justify-center",
            "rounded-none",
            "border-[0.25px]",
            "border-black",
            "text-xl",
            "font-bold",
            "hover:border-black" // override base styles
        ),
        extra_classes
    ])
}
