#[macro_use]
extern crate log;

pub mod domain;
pub mod statistics;
#[cfg(feature="gui")]
pub mod graphics;


#[cfg(test)]
mod tests {
    use domain::*;
    use domain::Point;
    use domain::terrain::Terrain;
    use domain::person::Person;

    #[test]
    fn declare_small_terrain() {
        let terrain: Terrain = Terrain::new(XSIZE, YSIZE);//[[0; YSIZE]; XSIZE];
        println!("terrain : \n{}", terrain);
    }

    #[test]
    fn declare_small_terrain_with_obstacle() {
        let mut terrain: Terrain = Terrain::new(XSIZE, YSIZE);//[[0; YSIZE]; XSIZE];
        terrain.add_obstacle(Point{x:1, y:1}, Point{x:2,y:2}); //add_rectangle(&mut terrain, (1, 1), (2, 2));
        println!("terrain : \n{}", terrain);
    }

    #[test]
    fn declare_sample_terrain() {
        let mut terrain: Terrain = Terrain::new_sample(XSIZE, YSIZE);
        println!("sample terrain : \n{}", terrain);
    }

    #[test]
    fn get_5_free_points_on_sample_terrain() {
        let mut terrain: Terrain = Terrain::new_sample(XSIZE, YSIZE);
        for i in 0..5 {
            let pt : Point = terrain.get_random_free_point()
                .expect("not enough free positions");
            terrain.set_pt_val(&pt, 51); // occupy }
        }
        assert!(terrain.count_persons_in_terrain() == 5);
        println!("occupied terrain : \n{}", terrain);
    }

    #[test]
    fn declare_sample_terrain_one_person() {
        let mut terrain: Terrain = Terrain::new_sample(XSIZE, YSIZE);

        let mut userX = Person::new(51, Point{x:0, y:0} );
        println!("userX : {}", userX);
        println!("terrain : \n{}", terrain); // no user on terrain : normal

        userX.place_on_terrain(&mut terrain);

        userX.move_to(&mut terrain, &Point{x:3, y:0});
        println!("userX : {}", userX);
        println!("terrain : \n{}", terrain); // user on terrain at (3,0) : normal
    }


    #[test]
    fn declare_sample_terrain_10_persons() {
        let mut terrain: Terrain = Terrain::new_sample(XSIZE, YSIZE);
        let nb_persons : usize = 10;
        #[derive(Debug)]
        let mut persons : Vec<Person>= Vec::with_capacity(nb_persons as usize);


        for i in 0..nb_persons {
            let pt : Point = terrain.get_random_free_point()
                .expect("not enough free positions");
            persons.push(Person::new( i, pt));
            terrain.set_pt_val(&persons[i as usize].position, 51); // occupy }
        }
        assert!(terrain.count_persons_in_terrain() == nb_persons);
        println!("persons array : {:?}", persons );
        println!("occupied terrain : \n{}", terrain);
    }


    #[test]
    fn move_iterations_on_small_terrain(){
        let mut terrain: Terrain = Terrain::new(XSIZE, YSIZE);//[[0; YSIZE]; XSIZE];
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



