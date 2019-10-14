struct Point { x: i32, y: i32, z: i32 }

fn main() {
    let mut point = Point { x: 0, y: 0, z: 0 };

    {
        let borrowed_point = &point;
        let another_borrow = &point;

        // Data can be accessed via the references and the original owner
        println!("Point has coordinates: ({}, {}, {})",
                borrowed_point.x, another_borrow.y, point.z);

        // Error! Can't borrow `point` as mutable because it's currently
        // borrowed as immutable.
        // let mutable_borrow = &mut point;
        // TODO ^ Try uncommenting this line.

        // Immutable references go out of scope.
    }

    {
        let mutable_borrow = &mut point;

        // Change date via mutable reference.
        mutable_borrow.x = 5;
        mutable_borrow.y = 2;
        mutable_borrow.z = 1;

        // Error! Can't borrow `point` as immutable because it's currently
        // borrowed as mutable.
        // let y = &point.y;
        // TODO ^ Try uncommenting this line.

        // Error! Cant't print because `println!` takes and immutable reference.
        // println!("Point Z coordinate is {}", point.z);
        // TODO ^ Try uncommenting this line.
        
        // Ok! Mutable references can be passed as immutable to `printnl!`.
        println!("Point has coordinates: ({}, {}, {})",
                    mutable_borrow.x, mutable_borrow.y, mutable_borrow.z);

        // Mutable reference goes out scope.
    }

    // Immutable references to `point` are allowed again.
    let borrowed_point = &point;
    println!("Point now has coordinates: ({}, {}, {})",
                borrowed_point.x, borrowed_point.y, borrowed_point.z);
}
