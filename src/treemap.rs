use std::{collections::HashMap, sync::Arc, usize};

type MapElement<T> = (Arc<str>, T, Vec<usize>, usize);

pub struct TreeMap<T> {
    elements: Vec<MapElement<T>>,
    map: HashMap<Arc<str>, usize>,
}

impl<T> TreeMap<T> {
    pub fn new(root_id: &str, element: T) -> TreeMap<T> {
        TreeMap { 
            elements: vec![(root_id.into(), element, vec![], usize::MAX)], 
            map: HashMap::from([(root_id.into(), 0)]),
        }
    }

    pub fn insert(&mut self, parent_id: &str, child_id: &str, element: T) {
        let parent_index = *self.map.get(parent_id).unwrap(); 

        self.elements.push((child_id.into(), element, vec![], parent_index));

        let child_index = self.elements.len() - 1;

        self.elements[parent_index].2.push(child_index);

        self.map.insert(child_id.into(), child_index);
    }

    //cant assume last isn't also being removed
    fn fill_empty(&mut self, replaced_idx: usize) {
        debug_assert!(self.elements.len() != 0);

        self.elements[replaced_idx] = self.elements.pop().unwrap();
        
        *self.map.get_mut(&self.elements[replaced_idx].0).unwrap() = replaced_idx;

        //update all children
        for child_idx in self.elements[replaced_idx].2.clone().iter() {
            self.elements[*child_idx].3 = replaced_idx;
        }

        let prev_last = self.elements.len();
        let parent_idx = self.elements[replaced_idx].3;

        //find in parent then update
        for child_idx in self.elements[parent_idx].2.iter_mut() {
            if *child_idx == prev_last {
                *child_idx = replaced_idx;
                break; 
            }
        }
    }

    pub fn remove(&mut self, element_id: &str) {
        let root_index = *self.map.get(element_id).unwrap();
        let parent_idx = self.elements[root_index].3;
        let mut root_index_index_in_parent = None;

        //find in parent then remove
        for (i, child_idx) in self.elements[parent_idx].2.iter().enumerate() {
            if *child_idx == root_index {
                root_index_index_in_parent = Some(i);
                break; 
            }
        }
        debug_assert_ne!(root_index_index_in_parent, None);
        self.elements[parent_idx].2.remove(root_index_index_in_parent.unwrap());

        //collect all to remove
        let mut removal_indicies = vec![];
        let mut stack = vec![root_index];
        while stack.len() > 0 {
            let index = stack.pop().unwrap();
            stack.append(&mut self.elements[index].2.clone());
            removal_indicies.push(index);
        }
        removal_indicies.sort_unstable();

        //replace with ones from end
        for i in (0..removal_indicies.len()).rev() {
            self.map.remove(&self.elements[removal_indicies[i]].0);

            if removal_indicies[i] == self.elements.len() - 1 {
                self.elements.pop();
            } else {
                self.fill_empty(removal_indicies[i]);
            }
        }
    }

    pub fn get(&self, element_id: &str) -> Option<&T> {
        let Some(idx) = self.map.get(element_id) else { return None };
        
        debug_assert!(*idx < self.elements.len());

        Some(&self.elements[*idx].1)
    }

    pub fn traverse<'a, U, F: FnMut(TreeTraverser<'a, T>) -> U>(&'a self, starting_id: &str, mut traverser: F) -> Option<U> {
        let Some(idx) = self.map.get(starting_id) else { return None };
        
        debug_assert!(*idx < self.elements.len());

        Some(traverser(TreeTraverser(self, *idx)))
    }
}

pub struct TreeTraverser<'a, T>(&'a TreeMap<T>, usize);

impl<'a, T> TreeTraverser<'a, T> {
    fn parent(&self) -> TreeTraverser<'a, T> {
        debug_assert_ne!(self.0.elements[self.1].3, usize::MAX);

        TreeTraverser(self.0, self.0.elements[self.1].3)
    }

    fn get_name(&self) -> Option<Arc<str>> {
        debug_assert!(self.1 < self.0.elements.len());

        Some(self.0.elements[self.1].0.clone())
    }
    
    fn children(&self) -> impl Iterator<Item = TreeTraverser<T>> {
        self.0.elements[self.1].2.iter().map(|idx| TreeTraverser(self.0, *idx))
    }
}

#[cfg(test)]
mod tests {
    use super::TreeMap;

    #[test]
    fn test_tree() {
        let mut tree_map = TreeMap::new("root", 32);        

        tree_map.insert("root", "seventeen", 17);
        assert_eq!(tree_map.get("seventeen"), Some(&17));

        tree_map.remove("seventeen");
        assert_eq!(tree_map.get("seventeen"), None);
        
        tree_map.insert("root", "seventeen", 17);
        tree_map.insert("seventeen", "eighteen", 18);
        tree_map.insert("eighteen", "nineteen", 19);
        tree_map.insert("nineteen", "twenty", 20);
        tree_map.insert("twenty", "twenty one", 21);

        assert_eq!(tree_map.get("seventeen"), Some(&17));
        assert_eq!(tree_map.get("eighteen"), Some(&18));
        assert_eq!(tree_map.get("nineteen"), Some(&19));
        assert_eq!(tree_map.get("twenty"), Some(&20));
        assert_eq!(tree_map.get("twenty one"), Some(&21));
        
        let deepest = tree_map.traverse("root", |traverse| {
            assert_eq!(traverse.get_name(), Some("root".into()));

            for child in traverse.children() {
                assert_eq!(Some("seventeen".into()), child.get_name());
                        
                let parent = child.parent();

                assert_eq!(Some("root".into()), parent.get_name());

                for child in child.children() {
                    assert_eq!(Some("eighteen".into()), child.get_name());

                    for child in child.children() {
                        assert_eq!(Some("nineteen".into()), child.get_name());

                        let parent = child.parent();

                        assert_eq!(Some("eighteen".into()), parent.get_name());

                        for child in child.children() {
                            assert_eq!(Some("twenty".into()), child.get_name());
                            
                            let parent = child.parent();

                            assert_eq!(Some("nineteen".into()), parent.get_name());

                            return child.get_name();
                        } 
                    } 
                } 
            }

            None
        }).unwrap();
        
        tree_map.remove("eighteen");
        
        assert_eq!(tree_map.get("seventeen"), Some(&17));
        assert_eq!(tree_map.get("eighteen"), None);
        assert_eq!(tree_map.get("nineteen"), None);
        assert_eq!(tree_map.get("twenty"), None);
        assert_eq!(tree_map.get("twenty one"), None);
    }
}

