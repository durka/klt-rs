use klt;
use std::{ptr, slice};
use std::os::raw;

pub unsafe fn unsafe_main() {
    let ft = klt::KLTReadFeatureTable(ptr::null_mut(), s!("features.txt"));
    let ftr = &mut *ft;
    let fl = klt::KLTCreateFeatureList(ftr.n_features);
    klt::KLTExtractFeatureList(fl, ft, 1);
    klt::KLTWriteFeatureList(fl, s!("feat1.txt"), s!("%3d"));
    klt::KLTReadFeatureList(fl, s!("feat1.txt"));
    klt::KLTStoreFeatureList(fl, ft, 2);
    klt::KLTWriteFeatureTable(ft, s!("ft2.txt"), s!("%3d"));

    let fh = klt::KLTCreateFeatureHistory(ftr.n_frames);
    let fhr = &mut *fh;
    klt::KLTExtractFeatureHistory(fh, ft, 5);

    println!("The feature history of feature number 5:\n");
    for i in 0..fhr.n_frames {
        println!("{}: ({:#5.1},{:#5.1}) = {}",
                 i, idx!(fhr.feature[i, n_frames].x), idx!(fhr.feature[i, n_frames].y),
                 idx!(fhr.feature[i, n_frames].val));
    }

    klt::KLTStoreFeatureHistory(fh, ft, 8);
    klt::KLTWriteFeatureTable(ft, s!("ft3.txt"), s!("%6.1f"));
}

