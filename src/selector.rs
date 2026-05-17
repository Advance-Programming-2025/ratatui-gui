use ratatui::widgets::{TableState};

enum SelectorState{
    Planet,
    Explorer,
}

pub(crate) struct Selector{
    selector_state: SelectorState,
    pub(crate)planet_selector: TableState,
    pub(crate)explorer_selector: TableState,

    last_planet_selected: Option<usize>,
    last_explorer_selected:Option<usize>,
    len_planets:usize,
    len_explorers:usize,
}

impl Selector{
    pub(crate)fn new(len_planets:usize, len_explorers:usize)->Self{
        Self { 
            selector_state: SelectorState::Planet, 
            planet_selector: TableState::default(), 
            explorer_selector: TableState::default(), 
            last_planet_selected: None, 
            last_explorer_selected: None, 
            len_planets,
            len_explorers,
        }
    }

    pub(crate)fn go_down(&mut self){
        match self.selector_state{
            SelectorState::Planet=>{
                match self.planet_selector.selected(){
                    Some(id)=>{
                        if id+1 < self.len_planets{
                            self.planet_selector.select(Some(id+1));
                            self.last_planet_selected = self.planet_selector.selected();
                        }
                    },
                    None=>{
                        self.planet_selector.select(Some(0));
                        self.last_planet_selected = self.planet_selector.selected();
                    },
                }
            },
            SelectorState::Explorer=>{
                match self.explorer_selector.selected(){
                    Some(id)=>{
                        if id+1 < self.len_explorers{
                            self.explorer_selector.select(Some(id+1));
                            self.last_explorer_selected = self.explorer_selector.selected();
                        }
                    },
                    None=>{self.explorer_selector.select(Some(0));
                            self.last_explorer_selected = self.explorer_selector.selected()},
                }
            }
        }
        
    }
    pub(crate)fn go_up(&mut self){
        match self.selector_state{
            SelectorState::Planet=>{
                match self.planet_selector.selected(){
                    Some(id)=>{
                        if id > 0{
                            self.planet_selector.select(Some(id-1));
                            self.last_planet_selected = self.planet_selector.selected();
                        }
                    },
                    None=>{
                        self.planet_selector.select(Some(0));
                        self.last_planet_selected = self.planet_selector.selected();
                    }
                }
            },
            SelectorState::Explorer=>{
                match self.explorer_selector.selected(){
                    Some(id)=>{
                        if id > 0{
                            self.explorer_selector.select(Some(id-1));
                            self.last_explorer_selected = self.explorer_selector.selected();
                        }
                    },
                    None=>{
                        self.explorer_selector.select(Some(0));
                        self.last_explorer_selected = self.explorer_selector.selected();
                    },
                }
            },
        }    
    }

    pub(crate)fn go_right(&mut self){
        match (&mut self.planet_selector.selected(), &mut self.explorer_selector.selected()){
            (None, None)=> self.go_down(),
            (Some(_), None)=>{
                self.selector_state = SelectorState::Explorer;
                self.planet_selector.select(None);
                
                match self.last_explorer_selected{
                    Some(id)=>self.explorer_selector.select(Some(id)),
                    None=>self.go_down(),
                }
            },
            (_, _)=>{},
        }
    }
    pub(crate)fn go_left(&mut self){
        match (&self.planet_selector.selected(), &self.explorer_selector.selected()){
            (None, None)=> self.go_down(),
            (None, Some(_))=>{
                self.selector_state = SelectorState::Planet;
                self.explorer_selector.select(None);
                match self.last_planet_selected{
                    Some(id)=>self.planet_selector.select(Some(id)),
                    None=>self.go_down(),
                }
            },
            (_, _)=>{},
        }
    }

    pub(crate)fn get_planet_selected(&self)->Option<usize>{
        self.planet_selector.selected()
    }
    pub(crate)fn get_explorer_selected(&self)->Option<usize>{
        self.explorer_selector.selected()
    }

    pub(crate)fn get_last_planet_selected(&self)->Option<usize>{
        self.last_planet_selected
    }
    pub(crate)fn get_last_explorer_selected(&self)->Option<usize>{
        self.last_explorer_selected
    }
}