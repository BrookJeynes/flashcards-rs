use tui::widgets::ListState;

pub struct StatefulList<T> {
    pub state: ListState,
    pub items: Vec<T>,
}

impl<T> StatefulList<T> {
    /// Create a StatefulList with the items passed in.
    pub fn with_items(items: Vec<T>) -> Self {
        let mut stateful_list = Self {
            state: ListState::default(),
            items,
        };


        // Auto select first item in decks list
        stateful_list.next();

        stateful_list
    }

    /// Move the internally selected item forward.
    pub fn next(&mut self) {
        if !self.items.is_empty() {
            let i = match self.state.selected() {
                Some(i) => {
                    if i >= self.items.len() - 1 {
                        i
                    } else {
                        i + 1
                    }
                }
                None => 0,
            };

            self.state.select(Some(i))
        }
    }

    /// Move the internally selected item backwards.
    pub fn previous(&mut self) {
        if !self.items.is_empty() {
            let i = match self.state.selected() {
                Some(i) => {
                    if i == 0 {
                        i
                    } else {
                        i - 1
                    }
                }
                None => 0,
            };

            self.state.select(Some(i))
        }
    }

    /// Delete an item by its index.
    pub fn delete(&mut self, index: usize) {
        if !self.items.is_empty() {
            self.items.remove(index);

            if !self.items.is_empty() && index > self.items.len() - 1 {
                self.previous();
            }
        }
    }

    /// Insert an item at the specified index position.
    pub fn insert(&mut self, new_item: T, index: usize) {
        self.items.insert(index, new_item)
    }

    /// Insert an item to the end of the list.
    pub fn push(&mut self, new_item: T) {
        self.items.push(new_item)
    }

    /// Return the current selected item.
    pub fn selected(&self) -> Option<usize> {
        if self.items.is_empty() {
            return None;
        }

        self.state.selected()
    }
}
