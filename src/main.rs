#[allow(dead_code)]
struct Bitstream {
    value: u16,
    width: u16,
}

// 6.1 Frame syntax
#[allow(dead_code)]
fn frame(sz: u32) {
    // startBitPos = get_position()
    // uncompressed_header()
    // trailing_bits()
    // if ( header_size_in_bytes == 0) {
    //   while ( get_position() < startBitPos + 8 *sz )
    //      padding_bit # f(1)
    //   return
    // }
    // load_probs( frame_context_idx )
    // load_probs2( frame_context_idx )
    // clear_counts()
    // init_bool( header_size_in_bytes )
    // exit_bool()
    // endBitPos = get_position()
    // headerBytes = (endBitPos - startBitPos) / 8
    // decode_tiles(sz - headerBytes)
    // refresh_probs()
}

// 6.1.1 Trailing bits syntax
#[allow(dead_code)]
fn trailing_bits() {
    // while ( get_position() & 7 )
    //   zero_bit # f(1)
}

// 6.1.2 Refresh probs syntax
#[allow(dead_code)]
fn refresh_probs() {
    // if ( error_resilient_mode == 0 && frame_parallel_decoding_mode == 0 ) {
    //   load_probs( frame_context_idx )
    //   adapt_coef_probs()
    //   if ( FrameIsIntra == 0 ) {
    //      load_probs2( frame_context_idx )
    //      adapt_noncoef_probs()
    //   }
    // }
    // if ( refresh_frame_context )
    //   save_probs( frame_context_idx )
}

// 6.2 Uncompressed header syntax
#[allow(dead_code)]
fn uncompressed_header() {
    // frame_marker     # f(2)
    // profile_low_bit  # f(1)
    // profile_high_bit # f(1)
    // Profile = (profile_high_bit << 1) + profile_low_bit
    // if (Profile == 3)
    //   reserved_zero  # f(1)
    // show_existing_frame  # f(1)
    // if ( show_existing_frame == 1) {
    //   frame_to_show_map_idx # f(3)
    //   header_size_in_bytes = 0
    //   refresh_frame_flags = 0
    //   loop_filter_level = 0
    //   return
    // }
    // LastFrameType = frame_type
    // frame_type       # f(1)
    // show_frame       # f(1)
    // error_resilient_mode # f(1)
    // if ( frame_type == KEY_FRAME ) {
    //   frame_sync_code()
    //   color_config()
    //   frame_size()
    //   render_size()
    //   refresh_frame_flags = 0xFF
    //   FrameIsIntra = 1
    // } else {
    //   if ( show_frame == 0 ) {
    //     intra_only   # f(1)
    //   } else {
    //     intra_only = 0
    //   }
    //   FrameIsIntra = intra_only
    //   if ( error_resilient_mode ) {
    //     reset_frame_context # f(2)
    //   } else {
    //     reset_frame_context = 0
    //   }
    //   if ( intra_only == 1 ) {
    //     frame_sync_code()
    //     if ( Profile > 0 ) {
    //       color_config()
    //     } else {
    //       color_space = CS_BT_601
    //       subsampling_x = 1
    //       subsampling_y = 1
    //       BitDepth = 8
    //     }
    //     refresh_frame_flags  # f(8)
    //     frame_size()
    //     render_size()
    //   } else {
    //     refresh_frame_flags  # f(8)
    //     for ( i = 0; i < 3; i++ ) {
    //       ref_frame_idx[i]                   # f(3)
    //       ref_frame_sign_bias[LAST_FRAME+i]  # f(1)
    //     }
    //     frame_size_with_refs()
    //     allow_high_precision_mv  # f(1)
    //     read_interpolation_filter()
    //   }
    // }
    // if ( error_resilient_mode == 0 ) {
    //   refresh_frame_context          # f(1)
    //   frame_parallel_decoding_mode   # f(1)
    // } else {
    //   refresh_frame_context = 0
    //   frame_parallel_decoding_mode = 1
    // }
    // frame_context_idx # f(2)
    // if ( FrameIsIntra || error_resilient_mode ) {
    //   setup_past_independence()
    //   if ( frame_type == KEY_FRAME || error_resilient_mode == 1
    //                                || reset_frame_context == 3) {
    //     for ( i = 0; i < 4; i++ ){
    //       save_probs(i)
    //     }
    //   } else if ( reset_frame_context == 2 ) {
    //     save_probs( frame_context_idx )
    //   }
    //   frame_context_idx = 0
    // }
    // loop_filter_params()
    // quantization_params()
    // segmentation_params()
    // tile_info()
    // header_size_in_bytes # f(16)
}

// 6.2.1 Frame sync syntax
#[allow(dead_code)]
fn frame_sync_code() {
    // frame_sync_byte_0    # f(8)
    // frame_sync_byte_1    # f(8)
    // frame_sync_byte_2    # f(8)
}

// 6.2.2 Color config syntax
#[allow(dead_code)]
fn color_config() {
    // if ( Profile >= 2 ) {
    //   ten_or_twelve_bit  # f(1)
    //   BitDepth = ten_or_twelve_bit ? 12 : 10
    // } else {
    //   BitDepth = 8
    // }
    // color_space  # f(3)
    // if ( color_space != CS_RGB ) {
    //   color_range    # f(1)
    //   if ( Profile == 1 || Profile == 3 ) {
    //     subsampling_x    # f(1)
    //     subsampling_y    # f(1)
    //     reserved_zero    # f(1)
    //   } else {
    //     subsampling_x = 1
    //     subsampling_y = 1
    //   }
    // } else {
    //   color_range = 1
    //   if ( Profile == 1 || Profile == 3 ) {
    //     subsampling_x = 0
    //     subsampling_y = 0
    //     reserved_zero    # f(1)
    //   }
    // }
}

// 6.2.3 Frame size syntax
#[allow(dead_code)]
fn frame_size() {
    // frame_width_minus_1  # f(16)
    // frame_height_minus_1 # f(16)
    // FrameWidth = frame_width_minus_1 + 1
    // FrameHeight = frame_height_minus_1 + 1
    // compute_image_size()
}

// 6.2.4 Render size syntax
#[allow(dead_code)]
fn render_size() {
    // render_and_frame_size_different  # f(1)
    // if ( render_and_frame_size_different == 1 ) {
    //   render_width_minus_1   # f(16)
    //   render_height_minus_1  # f(16)
    //   renderWidth = render_width_minus_1 + 1
    //   renderHeight = render_height_minus_1 + 1
    // } else {
    //   renderWidth = FrameWidth
    //   renderHeight = FrameHeight
    // }
}

// 6.2.5 Frame size with refs syntax
#[allow(dead_code)]
fn frame_size_with_refs() {
    // for ( i = 0; i < 3; i++ ) {
    //   found_ref  # f(1)
    //   if (found_ref == 1) {
    //     FrameWidth = RefFrameWidth[ ref_frame_idx[i] ]
    //     FrameHeight = RefFrameHeight[ ref_frame_idx[i] ]
    //     break
    //   }
    // }
    // if ( found_ref == 0 )
    //   frame_size()
    // else
    //   compute_image_size()
    // render_size()
}

// 6.2.6 Compute image size syntax
#[allow(dead_code)]
fn compute_image_size() {
    // MiCols = (FrameWidth + 7) >> 3
    // MiRows = (FrameHeight + 7) >> 3
    // Sb64Cols = (MiCols + 7) >> 3
    // Sb64Rows = (MiRows + 7) >> 3
}

// 6.2.7 Interpolation filter syntax
#[allow(dead_code)]
fn read_interpolation_filter() {
    // is_filter_switchable         # f(1)
    // if ( is_filter_switchable == 1 ) {
    //   Interpolation_filter = SWITCHABLE
    // } else {
    //   raw_interpolation_filter   # f(2)
    //   Interpolation_filter = literal_to_type[ raw_interpolation_filter ]
    // }
}

// 6.2.8 Loop filter params syntax
#[allow(dead_code)]
fn loop_filter_params() {
    // loop_filter_level            # f(6)
    // loop_filter_sharpness        # f(3)
    // loop_filter_delta_enabled    # f(1)
    // if ( loop_filter_delta_enabled == 1 ) {
    //   loop_filter_delta_update   # f(1)
    //   if ( loop_filter_delta_update == 1 ) {
    //     for ( i = 0; i < 4; i++ ) {
    //       update_ref_delta       # f(1)
    //       if ( update_ref_delta == 1 ) {
    //         loop_filter_ref_deltas[i]    # s(6)
    //       }
    //     }
    //     for ( i = 0; i < 2; i++ ) {
    //       update_mode_delta      # f(1)
    //       if ( update_mode_delta == 1 )
    //         loop_filter_mode_deltas[i]   # s(6)
    //     }
    //   }
    // }
}

// 6.2.9 Quantization params syntax
#[allow(dead_code)]
fn quantization_params() {
    // base_q_idx   # f(8)
    // delta_q_y_dc = read_delta_q()
    // delta_q_uv_dc = read_delta_q()
    // delta_q_uv_ac = read_delta_q()
    // Lossless = base_q_idx == 0 && delta_q_y_dc == 0
    //                              && delta_q_uv_dc == 0 && delta_q_uv_ac == 0
}



fn main() {
    // println!("Hello, world!");
}
