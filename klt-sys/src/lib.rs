extern crate libc;
use std::os::raw::{c_int, c_char, c_uchar, c_float, c_void};
pub use libc::FILE;

// types

pub type LocType   = c_float;
pub type PixelType = c_uchar;
pub type Bool      = c_int;

// structs

/// Define a struct and a type alias which is a mutable raw pointer to it
///
/// The struct will be #[repr(C)] and all members made pub
macro_rules! rec {
    ($name:ident: $recname:ident { $($elname:ident: $eltype:ty),* $(,)* }) => {
        #[repr(C)]
        #[allow(non_snake_case)]
        pub struct $recname {
            $(pub $elname: $eltype),*
        }

        pub type $name = *mut $recname;
    }
}

rec!{
    FloatImage: FloatImageRec {
        ncols: c_int,
        nrows: c_int,
        data: *mut c_float,
    }
}

rec! {
    TrackingContext: TrackingContextRec {
        mindist: c_int,
        window_width: c_int, window_height: c_int,

        sequential_mode: Bool,
        smooth_before_selecting: Bool,
        write_internal_images: Bool,
        lighting_insensitive: Bool,

        min_eigenvalue: c_int,
        min_determinant: c_float,
        min_displacement: c_float,
        max_iterations: c_int,
        max_residue: c_float,
        grad_sigma: c_float,
        smooth_sigma_fact: c_float,
        pyramid_sigma_fact: c_float,
        step_factor: c_float,
        n_skipped_pixels: c_int,
        borderx: c_int,
        bordery: c_int,
        n_pyramid_levels: c_int,
        subsampling: c_int,

        affine_window_width: c_int, affine_window_height: c_int,
        affine_consistency_check: c_int,
        affine_max_iterations: c_int,
        affine_max_residue: c_float,
        affine_min_displacement: c_float,
        affine_max_displacement_differ: c_float,

        pyramid_last: *mut c_void,
        pyramid_last_gradx: *mut c_void,
        pyramid_last_grady: *mut c_void,
    }
}

rec! {
    Feature: FeatureRec {
        x: LocType,
        y: LocType,
        val: c_int,	

        aff_img: FloatImage, 
        aff_img_gradx: FloatImage,
        aff_img_grady: FloatImage,
        aff_x: LocType,
        aff_y: LocType,
        aff_Axx: LocType,
        aff_Ayx: LocType,
        aff_Axy: LocType,
        aff_Ayy: LocType,
    }
}

rec! {
    FeatureList: FeatureListRec {
        n_features: c_int,
        feature: *mut Feature,
    }
}

rec! {
    FeatureHistory: FeatureHistoryRec {
        n_frames: c_int,
        feature: *mut Feature,
    }
}

rec! {
    FeatureTable: FeatureTableRec {
        n_frames: c_int,
        n_features: c_int,
        feature: *mut *mut Feature,
    }
}

// functions

extern "C" {
    // create
    pub fn KLTCreateTrackingContext() -> TrackingContext;
    pub fn KLTCreateFeatureList(nFeatures: c_int) -> FeatureList;
    pub fn KLTCreateFeatureHistory(nFrames: c_int) -> FeatureHistory;
    pub fn KLTCreateFeatureTable(nFrames: c_int, nFeatures: c_int) -> FeatureTable;

    // free
    pub fn KLTFreeTrackingContext(tc: TrackingContext);
    pub fn KLTFreeFeatureList(fl: FeatureList);
    pub fn KLTFreeFeatureHistory(fh: FeatureHistory);
    pub fn KLTFreeFeatureTable(ft: FeatureTable);

    // processing
    pub fn KLTSelectGoodFeatures(tc: TrackingContext, img: *const PixelType, ncols: c_int, nrows: c_int, fl: FeatureList);
    pub fn KLTTrackFeatures(tc: TrackingContext, img1: *const PixelType, img2: *const PixelType, ncols: c_int, nrows: c_int, fl: FeatureList);
    pub fn KLTReplaceLostFeatures(tc: TrackingContext, img: *const PixelType, ncols: c_int, nrows: c_int, fl: FeatureList);

    // utilities
    pub fn KLTCountRemainingFeatures(fl: FeatureList) -> c_int;
    pub fn KLTPrintTrackingContext(tc: TrackingContext);
    pub fn KLTChangeTCPyramid(tc: TrackingContext, search_range: c_int);
    pub fn KLTUpdateTCBorder(tc: TrackingContext);
    pub fn KLTStopSequentialMode(tc: TrackingContext);
    pub fn KLTSetVerbosity(verbosity: c_int);
    pub fn _KLTComputeSmoothSigma(tc: TrackingContext) -> c_float;

    // storing/extracting features
    pub fn KLTStoreFeatureList(fl: FeatureList, ft: FeatureTable, frame: c_int);
    pub fn KLTExtractFeatureList(fl: FeatureList, ft: FeatureTable, frame: c_int);
    pub fn KLTStoreFeatureHistory(fh: FeatureHistory, ft: FeatureTable, feat: c_int);
    pub fn KLTExtractFeatureHistory(fh: FeatureHistory, ft: FeatureTable, feat: c_int);

    // writing/reading
    pub fn KLTWriteFeatureListToPPM(fl: FeatureList, greyimg: *const PixelType, ncols: c_int, nrows: c_int, filename: *const c_char);
    pub fn KLTWriteFeatureList(fl: FeatureList, filename: *const c_char, fmt: *const c_char);
    pub fn KLTWriteFeatureHistory(fh: FeatureHistory, filename: *const c_char, fmt: *const c_char);
    pub fn KLTWriteFeatureTable(ft: FeatureTable, filename: *const c_char, fmt: *const c_char);
    pub fn KLTReadFeatureList(fl: FeatureList, filename: *const c_char) -> FeatureList;
    pub fn KLTReadFeatureHistory(fh: FeatureHistory, filename: *const c_char) -> FeatureHistory;
    pub fn KLTReadFeatureTable(ft: FeatureTable, filename: *const c_char) -> FeatureTable;

    // util
    pub fn _KLTCreateFloatImage(ncols: c_int, nrows: c_int) -> FloatImage;
    pub fn _KLTFreeFloatImage(img: FloatImage);
    pub fn _KLTPrintSubFloatImage(img: FloatImage, x0: c_int, y0: c_int, width: c_int, height: c_int);
    pub fn _KLTWriteFloatImageToPGM(img: FloatImage, filename: *const c_char);
    pub fn _KLTWriteAbsFloatImageToPGM(img: FloatImage, filename: *const c_char, scale: c_float);

    // pgm
    pub fn pgmReadFile(fname: *const c_char, img: *mut c_uchar, ncols: *mut c_int, nrows: *mut c_int) -> *mut c_uchar;
    pub fn pgmWriteFile(fname: *const c_char, img: *const c_uchar, ncols: c_int, nrows: c_int);
    pub fn ppmWriteFileRGB(fname: *const c_char, redimg: *const c_uchar, greenimg: *const c_uchar, blueimg: *const c_uchar, ncols: c_int, nrows: c_int);
    pub fn pgmRead(fp: *const FILE, img: *mut c_uchar, ncols: *mut c_int, nrows: *mut c_int) -> *mut c_uchar;
    pub fn pgmWrite(fp: *const FILE, img: *const c_uchar, ncols: c_int, nrows: c_int);
    pub fn ppmWrite(fp: *const FILE, redimg: *const c_uchar, greenimg: *const c_uchar, blueimg: *const c_uchar, ncols: c_int, nrows: c_int);
}

