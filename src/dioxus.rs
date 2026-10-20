use dioxus::prelude::*;

#[derive(Clone, Debug, PartialEq)]
struct Todo {
  id: usize,
  text: String,
  completed: bool,
}

#[component]
pub fn DioxusTodoApp() -> Element {
  // 1. Reactive Signals
  let mut todos = use_signal(Vec::<Todo>::new);
  let mut input_text = use_signal(String::new);

  // 2. Event Handlers
  let add_todo = move |ev: FormEvent| {
    if !input_text.read().trim().is_empty() {
      todos.write().push(Todo {
        id: todos.read().len(),
        text: input_text.read().clone(),
        completed: false,
      });
      input_text.set(String::new());
    }
  };

  let toggle_todo = move |id: usize| {
    if let Some(todo) = todos.write().iter_mut().find(|t| t.id == id) {
      todo.completed = !todo.completed;
    }
  };

  // 3. UI Template with Tailwind (RSX Syntax)
  rsx! {
    div { class: "max-w-md mx-auto mt-10 p-6 bg-slate-900 text-white rounded-xl shadow-lg border border-slate-800",
      h1 { class: "text-2xl font-bold mb-4 text-emerald-400", "Dioxus Task Board" }

      // Form Handling
      form { onsubmit: add_todo, class: "flex gap-2 mb-6",
        input {
          r#type: "text",
          class: "flex-1 px-4 py-2 bg-slate-800 rounded-lg border border-slate-700 focus:outline-none focus:border-emerald-500 text-white",
          placeholder: "Add a new task...",
          value: "{input_text}",
          oninput: move |ev| input_text.set(ev.value())
        }
        button {
          r#type: "submit",
          class: "px-4 py-2 bg-emerald-600 hover:bg-emerald-500 font-semibold rounded-lg transition-colors",
          "Add"
        }
      }

      // Conditionals & List Iteration
      div { class: "space-y-2",
        if todos.read().is_empty() {
          p { class: "text-slate-500 text-center py-4", "No tasks yet!" }
        } else {
          for todo in todos.read().iter().cloned() {
            div {
              key: "{todo.id}",
              onclick: move |_| toggle_todo(todo.id),
              class: if todo.completed {
                "flex items-center justify-between p-3 rounded-lg cursor-pointer transition-all bg-slate-800/50 line-through text-slate-500"
              } else {
                "flex items-center justify-between p-3 rounded-lg cursor-pointer transition-all bg-slate-800 text-slate-200 hover:bg-slate-750"
              },
              span { "{todo.text}" }
              span { class: "text-xs px-2 py-1 bg-slate-700 rounded-full",
                if todo.completed { "Done" } else { "Pending" }
              }
            }
          }
        }
      }
    }
  }
}