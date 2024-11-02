use std:{thread,time};
use std::sync::mpsc::channel;


// We will exploit the pattern multiple producers single consumers 
// Actually i need a supervisor that will monitor the position of every single bullet out there!!
struct shooter {
    position : i32,
    opponentPosition : i32,
    power : i32,
    totalOfShots : i32,
    right : bool,
}

struct bullet {
    PacePer500ms : i32, // by how much the 
    position : i32,
    direction : i32
}

impl shooter {
    fn strike( getOrder : channel::receiver<>,giveCoordinate : channel::Sender<>){
        let direction = 1
        if self.right{
            direction = -direction
        }
        let mut newBullet = bullet{
            PacePer500ms : self.power,
            position : self.position // initial position
            direction : direction
        }
        thread::spawn(move ||{
            // make the bullet move 
            let ten_millis = time::Duration::from_millis(500);

            while(direction ==1 && newBullet.position < self.opponentPosition) || (direction ==-1 && newBullet.position > self.opponentPosition){
                    newBullet.position += newBullet.PacePer500ms*direction; // increase th position
                    giveCoordinate.send(newBullet.position).unwrap();
                    let order = getOrder.try_recv(); // non-blocking waiaaatoitng
                    if let order = Some(kill){
                        if kill{
                            break;
                        }
                    }else{
                        break;
                    }

            }
        })
    }
}
struct terminal{
    Gameview : String,
    shooter1 : shooter,
    shooter2 : shooter,
    PositionUpdates : Vec<channel>,
    DestructionChannels : Vec<channel>
}
impl terminal{
    // First of things what does terminal do ? 
    // it's responsible of updating the screen evry 1ms
    // Monitoring the bullet to finish the game or kill the balls 
    // there are two cases where two bullet should die 
    // 1 -  if it collapes with another bullet on the road 
    // 2 -  it kills a shooter as a result the game ends
    fn new() -> terminal{
        let mut shooter_left = shooter{
        position : 1,
        opponentPosition : 10,
        power : 1,
        right : false,
    };
        let shooter_right = shooter{
        position : 10,
        opponentPosition : 1,
        power : 1,
        right : true, 
        }
        let mut initial__stand = format!("${}$"," ".repeat(shooter_right.position-shooter_left.position));
        terminal {
        GameState : true, // The game is app
        Gameview : String::from(initial__stand),
        shooter1 : shooter_left,
        shooter2 : shooter_right,
        PositionUpdates : Vec::new(),
        DestructionChannels : Vec::new()
        }
}
    fn render(){
        let mut newPositions = Vec::new();
        loop{
            for position in self.PositionUpdates{
                newPositions.push(position.try_recv().unwrap());
            }
            // render these new position 
            let current_view = String::from("$")
            current_view.sort();
            let previous = 0;
            for i in 0..current_view.len(){
                 current_view.push(" ".repeat(current_view.get(i).unwrap()));
                 current_view.push("@");
            }
            current_view.push("$");
            println!("{}",current_view);
            // Now lets check whether we have a collapse or something ??!!
            for i in 0..current_view.len(){
                let pos = current_view.get(i).unwrap();
                
            }
           }
    }
}
    
fn main(){

}
