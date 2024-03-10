use yew::prelude::*;

#[function_component(App)]
pub fn app() -> Html {
    let counter = use_state(|| 0);
    let onclick = {
        let counter = counter.clone();
        move |_| {
            let value = *counter + 1;
            counter.set(value);
        }
    };

    html! {
        <>
        <div class="flex justify-center items-center h-screen">
            <button class="bg-blue-500 hover:bg-blue-700 text-white font-bold py-2 px-4 rounded" {onclick}>{ "+1" }</button>
            <p class={classes!("mx-20")}>{ *counter }</p>
            <p class={classes!("bg-red-100")}>{"Test!"}</p>
            
                <h1 class="text-3xl">{"Hello, Tailwind CSS!"}</h1>
                <button class="bg-blue-500 hover:bg-blue-700 text-white font-bold py-2 px-4 rounded">
                    {"Button"}
                </button>
            </div>
        </>
    }
}
