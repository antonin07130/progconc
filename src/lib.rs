pub mod domain;

#[cfg(test)]
mod tests {
    use domain::Point;
    use domain::terrain::Terrain;
    use domain::person::Person;
    //println!("☠ // Concurrent escape (disaster at Cap3000) // ☠");

    #[test]
    fn declare_small_terrain() {
        let terrain: Terrain = Terrain::new();//[[0; YSIZE]; XSIZE];
        println!("terrain : \n{}", terrain);
    }

    #[test]
    fn declare_small_terrain_with_obstacle() {
        let mut terrain: Terrain = Terrain::new();//[[0; YSIZE]; XSIZE];
        terrain.add_obstacle(Point{x:1, y:1}, Point{x:2,y:2}); //add_rectangle(&mut terrain, (1, 1), (2, 2));
        println!("terrain : \n{}", terrain);
    }

    #[test]
    fn declare_small_terrain_with_obstacle_one_person() {
        let mut terrain: Terrain = Terrain::new();//[[0; YSIZE]; XSIZE];
        terrain.add_obstacle(Point{x:1, y:1}, Point{x:2,y:2}); //add_rectangle(&mut terrain, (1, 1), (2, 2));


        let mut userX = Person::new(51, Point{x:0, y:0} );
        println!("userX : {}", userX);
        println!("terrain : \n{}", terrain); // no user on terrain : normal

        userX.place_on_terrain(&mut terrain);

        userX.move_to(&mut terrain, &Point{x:3, y:0});
        println!("userX : {}", userX);
        println!("terrain : \n{}", terrain); // user on terrain at (3,0) : normal
    }


    #[test]
    fn move_iterations_on_small_terrain(){
        let mut terrain: Terrain = Terrain::new();//[[0; YSIZE]; XSIZE];
        terrain.add_obstacle(Point{x:1, y:1}, Point{x:2,y:2}); //add_rectangle(&mut terrain, (1, 1), (2, 2));


        let mut userX = Person::new(51, Point{x:3, y:0} );
        println!("userX : {}", userX);
        userX.place_on_terrain(&mut terrain);
        println!("terrain : \n{}", terrain);

        for n in 1..10 {
            println!("----------------------------------------- \n \n");
            println!("initial terrain : \n{}", terrain);
            #[derive(Debug)]
            //let mut good_point :&Point;
            let moves : Vec<Point>;
            moves = terrain.list_possible_moves(&userX.position);
            println!("possible moves : {:?}", moves);
            //#[derive(Debug)]
            let good_point = userX.choose_best_move(&moves);
            userX.move_to(&mut terrain, &good_point);
            println!("moving to : {}", good_point);
        }
    }


}


