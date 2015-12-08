use klt;
use std::{ptr, slice};
use std::os::raw;


pub unsafe fn unsafe_main() {
    let n_features = 100;

    let tc = klt::KLTCreateTrackingContext();
    klt::KLTPrintTrackingContext(tc);
    let fl = klt::KLTCreateFeatureList(n_features);
    let flr = &mut *fl;

    let mut ncols = 0;
    let mut nrows = 0;
    let img1 = klt::pgmReadFile(s!("img0.pgm"), ptr::null(), &mut ncols, &mut nrows);
    let img2 = klt::pgmReadFile(s!("img1.pgm"), ptr::null(), &mut ncols, &mut nrows);

    klt::KLTSelectGoodFeatures(tc, img1, ncols, nrows, fl);

    println!("In first image:");
    for i in 0..flr.n_features {
        println!("Feature #{}:  ({},{}) with value of {}",
                 i, idx!(flr.feature[i, n_features].x), idx!(flr.feature[i, n_features].y),
                 idx!(flr.feature[i, n_features].val));
    }

    klt::KLTWriteFeatureListToPPM(fl, img1, ncols, nrows, s!("feat1.ppm"));
    klt::KLTWriteFeatureList(fl, s!("feat1.txt"), s!("%3d"));

    klt::KLTTrackFeatures(tc, img1, img2, ncols, nrows, fl);

    println!("\nIn second image:");
    for i in 0..flr.n_features {
        println!("Feature #{}:  ({},{}) with value of {}\n",
                 i, idx!(flr.feature[i, n_features].x), idx!(flr.feature[i, n_features].y),
                 idx!(flr.feature[i, n_features].val));
    }

    klt::KLTWriteFeatureListToPPM(fl, img2, ncols, nrows, s!("feat2.ppm"));
    klt::KLTWriteFeatureList(fl, s!("feat2.fl"), ptr::null());  // binary file
    klt::KLTWriteFeatureList(fl, s!("feat2.txt"), s!("%5.1f")); // text file
}

