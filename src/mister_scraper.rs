
use reqwest::{self, Error};

pub struct Scraper {
    pub url : String,
}

impl Scraper {
        
    pub async fn scrape(&self) -> Result<String, Error> {
         
        let res = reqwest::get(&self.url)
            .await?
            .text()
            .await?
            .replace("\n", "")
            .replace("\t", ""); 

        Ok(res)
    }

    pub fn get_text(&self, mut html : String) -> String { 

        let mut return_string : String = String::new();

        while html.contains(">") || html.contains("<") {
            
            // approach is going to be getting text between "><" markers,
            // then deleting, updating return_string and starting again

            let opening_occurence = html.find(">").unwrap(); 
            // first occurence of desired start text marker

            let mut temp_html = html.clone();
            // temp html String to delete everything before desired open mark 

            temp_html.replace_range(0..=opening_occurence, "");
            // delete everything before desired marker

            let closing_occurance = temp_html.find("<").unwrap_or_default();
             
            temp_html.replace_range(closing_occurance..temp_html.len(), ""); 
            // delete everything after closing marker    
            
            // leaving anything between the child element alone

            return_string = return_string + temp_html.as_str();
            // update return_string 

            html.replace_range(0..=closing_occurance.clone(), "");
            // update html 
        }
        
        return return_string;

    } 
    

    pub fn get_element(&self, id : String, html : String) -> String {
        
        let id_occurence = html.find(id.as_str()); // find ID in html
       
        if id_occurence == None { // error handling
            return String::from("Element cannot be found.");
        }
         
        let id_occurence = id_occurence.unwrap(); // unwrap index value 

        let mut start_tag_pos = 0; // index of the start tag

        // walk back to get index of the start tag
        for i in 0..=id_occurence {
            let check_val_at = id_occurence - i; // current index that we are checking
            
            if html.chars().nth(check_val_at).unwrap() == '<' {
                start_tag_pos = check_val_at;
                break; 
            }
        }
        
        // check for self closing tag here
        
        let mut self_close_pointer = start_tag_pos;        

        while &self_close_pointer < &html.len() {
            
            // if we see this marker before '/' then we know its not self closing
            if &html.chars().nth(self_close_pointer).unwrap() == &'>' {
                break;
            }
             
            if &html.chars().nth(self_close_pointer).unwrap() == &'/' {
                
                while &self_close_pointer < &html.len() {
                    
                    if &html.chars().nth(self_close_pointer).unwrap() == &'>' {
                        let close_tag = self_close_pointer;
                        
                        return html[start_tag_pos..=close_tag].to_string();
                    }
                    
                    else {
                        self_close_pointer += 1;
                    }
                }

            }
            
            self_close_pointer += 1;
        }



        let mut temp_html = html.clone(); // clone string for throwaway test 

        temp_html.replace_range(0..start_tag_pos, ""); // isolate from beginning tag
        
        // get tag type Ex: <div 
        let tag_type = temp_html.split(" ").collect::<Vec<&str>>().get(0).unwrap().to_string(); 
                                                                                        
        let mut closing_tag = tag_type.split("").collect::<Vec<&str>>();
        closing_tag.push(">");
        closing_tag.insert(2, "/");
        let closing_tag = closing_tag.join(""); 
        // this is the closing tag of the choosen element

        drop(temp_html);
       
        let mut num_of_opening_tags = 1; 
        
        let mut search_pointer = start_tag_pos; // start search point
        
        let range = closing_tag.len(); // length of closing tag
        

        while &search_pointer < &(&html.len()-range) { 
            
            let test_section = utf8_slice::slice(&html.as_str(), search_pointer, search_pointer+range).to_string();
            if  &test_section == &closing_tag {
                
                // index of end of selected substring
                let pos_end_tag = search_pointer+range; 
                
                let search_field = &html[start_tag_pos..pos_end_tag];
                
                // number of opening tags in search_field
                let count : Vec<&str> = search_field.matches(tag_type.as_str()).collect();
                

                if num_of_opening_tags == count.len() as i32 {
                    return search_field.to_string(); 
                }
                
                else {
                    num_of_opening_tags += 1; 
                    search_pointer += 1;
                }
            } 

            else {
                search_pointer += 1;
            } 
                
        }

        return String::from("no element found, or bad html formatting");
    }


    pub fn get_elements(&self, id: String, mut html : String) -> Vec<String> {
        
        let mut return_value : Vec<String> = vec![];

        while (&html.contains((&id).as_str())).to_owned() {
            
            let first_ele = self.get_element((&id).to_string(), (&html).to_string());

            return_value.push((&first_ele).to_string());
            
            html = utf8_slice::slice(&html.as_str(), first_ele.len()+&html.find(first_ele.as_str()).unwrap(), html.len()).to_string(); 

        }
               
        return return_value; 

    }

}

