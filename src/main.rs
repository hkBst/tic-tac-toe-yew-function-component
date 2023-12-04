use tic_tac_toe::*;
use yew::prelude::*;

fn main() {
    yew::Renderer::<App>::new().render();
}

#[function_component]
fn App() -> Html {
    let game = use_state(Game::new);

    let board = [Hor::Left, Hor::Mid, Hor::Right].into_iter().flat_map(|h| {
        [Vert::Top, Vert::Mid, Vert::Bottom].into_iter().map({
            let game = game.clone();
            move |v| {
                let action = {
                    let game = game.clone();
                    Callback::from(move |_| {
                        game.set({
                            let mut g = (*game).clone();
                            g.act(FieldName { v, h });
                            g
                        });
                    })
                };
                let state = game.get(FieldName { v, h });
                html! { <Field action={action} state={state}/> }
            }
        })
    });

    html! { <div class="game">
        <div class="board"> {board.collect::<Vec<_>>()} </div>
        <div class="state"> {game.state().to_string()} </div>
    </div> }
}

#[derive(Properties, PartialEq)]
pub struct FieldProps {
    pub action: Callback<MouseEvent>,
    pub state: FieldState,
}

#[function_component]
fn Field(FieldProps { action, state }: &FieldProps) -> Html {
    let class = match state.0 {
        Some(Side::X) => "cell side-X",
        Some(Side::O) => "cell side-O",
        None => "cell",
    };
    let value = state.to_string();

    html! { <div class={class} onclick={action}> {value} </div> }
}
