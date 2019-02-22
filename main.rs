#[derive(Debug)]


struct Pixel{
    r:u8,
    g:u8,
    b:u8,
}


impl IntoIterator for Pixel{
    type Item = u8;
    type IntoIter = PixelIntoIterator;
    
    fn into_iter(self) -> Self::IntoIter{
        PixelIntoIterator{
            pixel:self,
            index:0,
        }
    }
    
}


struct PixelIntoIterator{
    pixel:Pixel,
    index:u8,
}


impl Iterator for PixelIntoIterator{
    type Item = u8;
    
    fn next(&mut self) -> Option<u8>{
        
        
        let result = match self.index{
            0 => { self.pixel.r},
            1 => { self.pixel.g},
            2 => { self.pixel.b},
            _ => { return None},
        };
        
      self.index +=1;
        Some(result)
    }    
     
    
}

fn main(){
    let p = Pixel{
                r:45,
                g:34,
                b:33 };
                
    let p1 = Pixel{
                r:4,
                g:3,
                b:39 };
    
    let mut v = vec!();
    
        v.push(p);
        v.push(p1);
   
        //for loop on pixel
        println!("This is for loop on Pixel struct");
        
        
        for j in v{
        
            for i in j{
                println!("{}",i);
            }
        }
}
  