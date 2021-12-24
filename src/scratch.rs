    // let fourteen = 99999999999999 as i64;
    // let digits = (1..fourteen).into_par_iter().map(|x| {
    //     let s = format!("{:0>14}", x);
    //     s.chars()
    //         .map(|c| c.to_string().parse::<i32>().unwrap())
    //         .collect::<Vec<i32>>()
    // });

    // digits.for_each(|digit| {
    //     let state = execute(mints.clone(), digit.clone());
    //     if state.z == 0 {
    //         println!("{:?}", digit);
    //         dbg!(digit);
    //     }
    // });

    //dbg!(&sections[0]);
    //    dbg!(sections.len());
    //for i in 1..=9 {
    //    dbg!(i, execute(sections[0].clone(), vec![i]));
    //}
    // for section in &sections {
    //     dbg!(section.len());
    // }
    // for i in 0..18 {
    //     for j in 1..14 {
    //     if sections[0][i] != sections[j][i]{
    //         println!("The {} instruction differs between 0 and {}", i, j);
    //         println!("{:?}",&sections[0][i].args);
    //         println!("{:?}",&sections[j][i].args);
    //     }
    //     }
    // }
    // ;

        // let mut numbers = vec![vec![(0,0)]]; //previous digit, previous z_value
    // for i in (0..14).rev() {
    //     println!("{}",i);
    //     let mut new_numbers = vec![];
    //     for number in &numbers {
    //         println!("{:?}", number);
    //         let (_previous_digit, z_previous) = *number.last().unwrap();
    //         for digit in 1..9 {
    //             for next_z in 0..1000 {
    //                 let computed_z = func(digit, next_z, vs[i].clone());
    //                 if computed_z == z_previous {
    //                     let mut new_number = number.clone();
    //                     new_number.push((digit,next_z));
    //                     new_numbers.push(new_number)
    //                 }
    //             }
    //         }
    //     }
    //     numbers = new_numbers;
    // }