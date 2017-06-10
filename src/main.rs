#[allow(dead_code)]
struct Bitstream {
    value: u16,
    width: u16,
}

// 6.1 Frame syntax
#[allow(dead_code)]
fn frame(/*sz: u32*/) {
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

// 6.2.10 Delta quantizer syntax
#[allow(dead_code)]
fn read_delta_q() {
    // delta_coded  # f(1)
    // if ( delta_coded ) {
    //   delta_q    # f(4)
    // } else {
    //   delta_q = 0
    // }
    // return delta_q
}

// 6.2.11 Segmentation params syntax
#[allow(dead_code)]
fn segmentation_params() {
    // segmentation_enabled # f(1)
    // if ( segmentation_enabled == 1 ) {
    //   segmentation_update_map    # f(1)
    //   if ( segmentation_update_map == 1 ) {
    //     for ( i = 0; i < 7; i++ )
    //       segmentation_tree_probs[i] = read_prob()
    //     segmentation_temporal_update # f(1)
    //     for ( i = 0; i < 3; i++ )
    //       segmentation_pred_prob[i] = segmentation_temporal_update ? read_prob() : 255
    //   }
    //   segmentation_update_map    # f(1)
    //   if ( segmentation_update_data == 1 ) {
    //     segmentation_abs_or_delta_update # f(1)
    //     for ( i = 0; i < MAX_SEGMENTS; i++ ) {
    //       for ( j = 0; j < SEG_LVL_MAX; j++ ) {
    //         feature_value = 0
    //         feature_enabled  # f(1)
    //         FeatureEnabled[i][j] = feature_enabled
    //         if ( feature_enabled == 1 ) {
    //           bits_to_read = segmentation_feature_bits[j]
    //           feature_value  # f(bits_to_read)
    //           if ( segmentation_feature_signed[j] == 1 ) {
    //             feature_sign # f(1)
    //             if ( feature_sign == 1 )
    //               feature_value *= -1
    //           }
    //         }
    //         FeatureDelta[i][j] = feature_value
    //       }
    //     }
    //   }
    // }
}

// 6.2.12 Probability syntax
#[allow(dead_code)]
fn read_prob() {
    // prob_doded   # f(1)
    // if ( prob_coded ) {
    //   prob       # f(8)
    // } else {
    //   prob = 255
    // }
    // return prob
}

// 6.2.13 Tile info syntax
#[allow(dead_code)]
fn tile_info() {
    // minLog2TileCols = calc_min_log2_tile_cols()
    // maxLog2TileCols = calc_max_log2_tile_cols()
    // tile_cols_log2 = minLog2TileCols
    // while ( tile_cols_log2 < maxLog2TileCols ) {
    //   increment_tile_cols_log2   # f(1)
    //   if ( increment_tile_cols_log2 == 1 )
    //     tile_cols_log2++
    //   else
    //     break
    // }
    // tile_rows_log2   # f(1)
    // if ( tile_rows_log2 == 1 ) {
    //   increment_tile_rows_log2   # f(1)
    //   tile_rows_log2 += increment_tile_rows_log2
    // }
}

// 6.2.14 Tile size calculation
#[allow(dead_code)]
fn calc_min_log2_tile_cols() {
    // minLog2 = 0
    // while ( (MAX_TILE_WIDTH_B64 << minLog2) < Sb64Cols )
    //   minLog2++
    // return minLog2
}
#[allow(dead_code)]
fn calc_max_log2_tile_cols() {
    // maxLog2 = 1
    // while ( (Sb64Cols >> maxLog2) >= MIN_TILE_WIDTH_B64 )
    //   maxLog2++
    // return maxLog2 - 1
}

// 6.3 Compressed header syntax
#[allow(dead_code)]
fn compressed_header() {
    // read_tx_mode()
    // if ( tx_mode == TX_MODE_SELECT ) {
    //   tx_mode_probs()
    // }
    // read_coef_probs()
    // read_skip_prob()
    // if ( FrameIsIntra == 0 ) {
    //   read_inter_mode_probs()
    //   if ( interpolation_filter == SWITCHABLE )
    //     read_interp_filter_probs()
    //   read_is_inter_probs()
    //   frame_reference_mode()
    //   frame_reference_mode_probs()
    //   read_y_mode_probs()
    //   read_partition_probs()
    //   mv_probs()
    // }
}

// 6.3.1 Tx mode syntax
#[allow(dead_code)]
fn read_tx_mode() {
    // if ( Lossless == 1 ) {
    //   tx_mode = ONLY_4X4
    // } else {
    //   tx_mode    # L(2)
    //   if ( tx_mode == ALLOW_32X32 ) {
    //     tx_mode_select # L(1)
    //     tx_mode += tx_mode_select
    //   }
    // }
}

// 6.3.2 Tx mode probs syntax
#[allow(dead_code)]
fn tx_mode_probs() {
    // for ( i = 0; i < TX_SIZE_CONTEXTS; i++ )
    //   for ( j = 0; j < TX_SIZES - 3; j++ )
    //     tx_probs_8x8[i][j] = diff_update_prob(tx_probs_8x8[i][j])
    // for ( i = 0; i < TX_SIZE_CONTEXTS; i++ )
    //   for ( j = 0; j < TX_SIZES - 2; j++ )
    //     tx_probs_16x16[i][j] = diff_update_prob(tx_probs_16x16[i][j])
    // for ( i = 0; i < TX_SIZE_CONTEXTS; i++ )
    //   for ( j = 0; j < TX_SIZES - 1; j++ )
    //     tx_probs_32x32[i][j] = diff_update_prob(tx_probs_32x32[i][j])
}

// 6.3.3 Diff update prob syntax
#[allow(dead_code)]
fn diff_update_prob(/*prob: f32*/) {
    // update_prob  # B(252)
    // if ( update_prob == 1 ) {
    //   deltaProb = decode_term_subexp()
    //   prob = inv_remap_prob(deltaProb, prob)
    // }
    // return prob
}

// 6.3.4 Decode term subexp syntax
#[allow(dead_code)]
fn decode_term_subexp() {
    // bit # L(1)
    // if ( bit == 0 ) {
    //   sub_exp_val    # L(4)
    //   return sub_exp_val
    // }
    // bit # L(1)
    // if ( bit == 0 ) {
    //   sub_exp_val_minus16 # L(4)
    //   return sub_exp_val_minus16 + 16
    // }
    // bit # L(1)
    // if ( bit == 0 ) {
    //   sub_exp_val_minus32 # L(5)
    //   return sub_exp_val_minus32 + 32
    // }
    // v # L(5)
    // if ( v < 65 )
    //   return v + 64
    // bit # L(1)
    // return (v << 1) - 1 + bit
}

// 6.3.5 Inv remap prob syntax
#[allow(dead_code)]
fn inv_remap_prob(/*deltaProb: u32, prob: u32*/) {
    // m = prob
    // v = deltaProb
    // v = inv_map_table[v]
    // m--
    // if ( (m << 1) <= 255 )
    //   m = 1 + inv_recenter_nonneg(v, m)
    // else
    //   m = 255 - inv_recenter_nonneg(v, 255 - 1 - m)
    // return m
}

// 6.3.6 Inv recenter nonneg syntax
#[allow(dead_code)]
fn inv_recenter_nonneg(/*v: u32, m: u32*/) {
    // if ( v > 2 * m )
    //   return v
    // if ( v & 1 )
    //   return m - ((v+1) >> 1)
    // return m + (v >> 1)
}

// 6.3.7 Coef probs syntax
#[allow(dead_code)]
fn read_coef_probs() {
    // maxTxSize = tx_mode_to_biggest_tx_size[tx_mode]
    // for ( txSz = TX_4X4; txSz <= maxTxSize; txSz++ ) {
    //   update_probs   # L(1)
    //   if ( update_probs == 1 )
    //     for ( i = 0; i < 2; i++ )
    //       for ( j = 0; j < 2; j++ )
    //         for ( k = 0; k < 6; k++ )
    //           maxL = ( k == 0 ) ? 3 : 6
    //             for ( l = 0; l < maxL; l++ )
    //               for ( m = 0; m < 3; m++ )
    //                 coef_probs[txSz][i][j][k][l][m] = diff_update_prob(coef_probs[txSz][i][j][k][l][m])
    // }
}

// 6.3.8 Skip probs syntax
#[allow(dead_code)]
fn read_skip_prob() {
    // for ( i = 0; i < SKIP_CONTEXTS; i++ )
    //   skip_prob[i] = diff_update_prob(skip_prob[i])
}

// 6.3.9 Inter mode probs syntax
#[allow(dead_code)]
fn read_inter_mode_probs() {
    // for ( i = 0; i < INTER_MODE_CONTEXTS; i++ )
    //   for ( j = 0; j < INTER_MODES - 1; j++ )
    //     inter_mode_probs[i][j] = diff_update_prob(inter_mode_probs[i][j])
}

// 6.3.10 Interp filter probs syntax
#[allow(dead_code)]
fn read_interp_filter_probs() {
    // for ( j = 0; j < INTERP_FILTER_CONTEXTS; j++ )
    //   for ( i = 0; i < SWITCHABLE_FILTERS - 1; i++ )
    //     interp_filter_probs[j][i] = diff_update_prob(interp_filter_probs[j][i])
}

// 6.3.11 Intra inter probs syntax
#[allow(dead_code)]
fn read_is_inter_probs() {
    // for ( i = 0; i < IS_INTER_CONTEXTS; i++ )
    //   is_inter_prob[i] = diff_update_prob(is_inter_prob[i])
}

// 6.3.12 Frame reference mode syntax
#[allow(dead_code)]
fn frame_reference_mode() {
    // compoundReferenceAllowed = 0
    // for ( i = 1; i < REFS_PER_FRAME; i++ )
    //   if ( ref_frame_sign_bias[i+1] != ref_frame_sign_bias[1] )
    //     compoundReferenceAllowed = 1
    // if ( compoundReferenceAllowed == 1 ) {
    //   non_single_reference # L(1)
    //   if ( non_single_reference == 0 ) {
    //     reference_mode = SINGLE_REFERENCE
    //   } else {
    //     reference_select # L(1)
    //     if ( reference_select == 0 )
    //       reference_mode = COMPOUND_REFERENCE
    //     else
    //       reference_mode = REFERENCE_MODE_SELECT
    //     setup_compound_reference_mode()
    //   }
    // } else {
    //   reference_mode = SINGLE_REFERENCE
    // }
}

// 6.3.13 Frame reference mode probs syntax
#[allow(dead_code)]
fn frame_reference_mode_probs() {
    // if ( reference_mode == REFERENCE_MODE_SELECT ) {
    //   for ( i = 0; i < COMP_MODE_CONTEXTS; i++ )
    //     comp_mode_prob[i] = diff_update_prob(comp_mode_prob[i])
    // }
    // if ( reference_mode != COMPOUND_REFERENCE ) {
    //   for ( i = 0; i < REF_CONTEXTS; i++ ) {
    //     single_ref_prob[i][0] = diff_update_prob(single_ref_prob[i][0])
    //     single_ref_prob[i][1] = diff_update_prob(single_ref_prob[i][1])
    //   }
    // }
    // if ( reference_mode != SINGLE_REFERENCE ) {
    //   for ( i = 0; i < REF_CONTEXTS; i++ )
    //     comp_ref_prob[i] = diff_update_prob(comp_ref_prob[i])
    // }
}

// 6.3.14 Y mode probs syntax
#[allow(dead_code)]
fn read_y_mode_probs() {
    // for ( i = 0; i < BLOCK_SIZE_GROUPS; i++ )
    //   for ( j = 0; j < INTRA_MODES - 1; j++ )
    //     y_mode_probs[i][j] = diff_update_prob(y_mode_probs[i][j])
}

// 6.3.15 Partition probs syntax
#[allow(dead_code)]
fn read_partition_probs() {
    // for ( i = 0; i < PARTITION_CONTEXTS; i++ )
    //   for ( j = 0; j < PARTITION_TYPES - 1; j++ )
    //     partition_probs[i][j] = diff_update_prob(partition_probs[i][j])
}

// 6.3.16 MV probs syntax
#[allow(dead_code)]
fn mv_probs() {
    // for ( j = 0; j < MV_JOINTS - 1; j++ )
    //   mv_joint_probs[j] = update_mv_prob(mv_joint_probs[j])
    // for ( i = 0; i < 2; i++ ) {
    //   mv_sign_prob[i] = update_mv_prob(mv_sign_prob[i])
    //   for ( j = 0; j < MV_CLASSES - 1; j++ )
    //     mv_class_probs[i][j] = update_mv_prob(mv_class_probs[i][j])
    //   mv_class0_bit_prob[i] = update_mv_prob(mv_class0_bit_prob[i])
    //   for ( j = 0; j < MV_OFFSET_BITS; j++ )
    //     mv_bits_prob[i][j] = update_mv_prob(mv_bits_prob[i][j])
    // }
    // for ( i = 0; i < 2; i++ ) {
    //   for ( j = 0; j < CLASS0_SIZE; j++ )
    //     for ( k = 0; k < MV_FR_SIZE - 1; k++ )
    //       mv_class0_fr_probs[i][j][k] = update_mv_prob(mv_class0_fr_probs[i][j][k])
    //   for ( k = 0; k < MV_FR_SIZE - 1; k++ )
    //     mv_fr_probs[i][k] = update_mv_prob(mv_fr_probs[i][k])
    // }
    // if ( allow_high_precision_mv ) {
    //   for ( i = 0; i < 2; i++ ) {
    //     mv_class0_hp_prob[i] = update_mv_prob(mv_class0_hp_prob[i])
    //     mv_hp_prob[i] = update_mv_prob(mv_hp_prob[i])
    //   }
    // }
}

// 6.3.17 Update mv prob syntax
#[allow(dead_code)]
fn update_mv_prob(/* prob:u32 */) {
    // update_mv_prob # B(252)
    // if ( update_mv_prob == 1 ) {
    //   mv_prob # L(7)
    //   prob = (mv_prob << 1) | 1
    // }
    // return prob
}

// 6.3.18 Setup compound reference mode syntax
#[allow(dead_code)]
fn setup_compound_reference_mode() {
    // if ( ref_frame_sign_bias[LAST_FRAME] == ref_frame_sign_bias[GOLDEN_FRAME] ) {
    //   CompFixedRef = ALTREF_FRAME
    //   CompVarRef[0] = LAST_FRAME
    //   CompVarRef[1] = GOLDEN_FRAME
    // } else if ( ref_frame_sign_bias[LAST_FRAME] == ref_frame_sign_bias[ALTREF_FRAME] ) {
    //   CompFixedRef = GOLDEN_FRAME
    //   CompVarRef[0] = LAST_FRAME
    //   CompVarRef[1] = ALTREF_FRAME
    // } else {
    //   CompFixedRef = LAST_FRAME
    //   CompVarRef[0] = GOLDEN_FRAME
    //   CompVarRef[1] = ALTREF_FRAME
    // }
}

// 6.4 Decode tiles syntax
#[allow(dead_code)]
fn decode_tiles(/*sz*/) {
    // tileCols = 1 << tile_cols_log2
    // tileRows = 1 << tile_rows_log2
    // clear_above_context()
    // for ( tileRow = 0; tileRow < tileRows; tileRow++ ) {
    //   for ( tileCol = 0; tileCol < tileCols; tileCol++ ) {
    //     lastTile = (tileRow == tileRows - 1) && (tileCol == tileCols - 1)
    //     if ( lastTile ) {
    //       tile_size = sz
    //     } else {
    //       tile_size # f(32)
    //       sz -= tile_size + 4
    //     }
    //     MiRowStart = get_tile_offset(tileRow, MiRows, tile_rows_log2)
    //     MiRowEnd = get_tile_offset(tileRow+1, MiRows, tile_rows_log2)
    //     MiColStart = get_tile_offset(tileCol, MiCols, tile_cols_log2)
    //     MiColEnd = get_tile_offset(tileCol+1, MiCols, tile_cols_log2)
    //     init_bool(tile_size)
    //     decode_tile()
    //     exit_bool()
    //   }
    // }
}

// 6.4.1 Get tile offset syntax
#[allow(dead_code)]
fn get_tile_offset() {
    // sbs = (mis + 7) >> 3
    // offset = ((tileNum * sbs) >> tileSzLog2) << 3
    // return Min(offset)
}

// 6.4.2 Decode tile syntax
#[allow(dead_code)]
fn decode_tile() {
    // for ( r = MiRowStart; r < MiRowEnd; r += 8 ) {
    //   clear_left_context()
    //   for ( c = MiColStart; c < MiColEnd; c += 8 )
    //     decode_partition(r, c, BLOCK_64X64)
    // }
}

// 6.4.3 Decode partition syntax
#[allow(dead_code)]
fn decode_partition() {
    // if ( r >= MiRows || c >= MiCols)
    //     return 0
    // num8x8 = num_8x8_blocks_wide_lookup[bsize]
    // halfBlock8x8 = num8x8 >> 1
    // hasRows = (r + halfBlock8x8) < MiRows
    // hasCols = (c + halfBlock8x8) < MiCols
    // partition # T
    // subsize = subsize_lookup[partition][bsize]
    // if ( subsize < BLOCK_8X8 || partition == PARTITION_NONE ) {
    //     decode_block(r, c, subsize)
    // } else if ( partition == PARTITION_HORZ ) {
    //     decode_block(r, c, subsize)
    //     if (hasRows) {
    //         decode_block(r+halfBlock8x8, c, subsize)
    //     }
    // } else if ( partition == PARTITION_VERZT ) {
    //     decode_block(r, c, subsize)
    //     if (hasCols)
    //         decode_block(r, c+halfBlock8x8, subsize)
    // } else {
    //     decode_partition(r, c, subsize)
    //     decode_partition(r, c+halfBlock8x8, subsize)
    //     decode_partition(r+halfBlock8x8, c, subsize)
    //     decode_partition(r+halfBlock8x8, c+halfBlock8x8, subsize)
    // }
    // if (bsize==BLOCK_8X8 || partition != PARTITION_SPLIT) {
    //     for (i=0;i<num8x8;i++) {
    //         AbovePartitionContext[c+i] = 15 >> b_width_log2_lookup[subsize]
    //         LeftPartitionContext[r+i] = 15 >> b_height_log2_lookup[subsize]
    //     }
    // }
}

// 6.4.4 Decode block syntax
#[allow(dead_code)]
fn decode_block(/*r, c, subsize*/) {
    // MiRow = r
    // MiCol = c
    // MiSize = subsize
    // AvailU = r > 0
    // AvailL = c > MiColStart
    // mode_info()
    // EobTotal = 0
    // residual()
    // if (is_inter && subsize >= BLOCK_8X8 && EobTotal == 0) {
    //     skip = 1
    // }
    // for (y=0;y<num_8x8_blocks_high_lookup[subsize];y++)
    //     for (x=0;x<num_8x8_blocks_wide_lookup[subsize];x++) {
    //         Skips[r+y][c+x] = skip
    //         TxSizes[r+y][c+x] = tx_size
    //         MiSizes[r+y][c+x] = MiSize
    //         YModes[r+y][c+x] = y_mode
    //         SegmentIds[r+y][c+x] = segment_id
    //         for (refList=0;refList<2;refList++)
    //             RefFrames[r+y][c+x][refList] = ref_frame[refList]
    //         if (is_inter) {
    //             InterpFilters[r+y][c+x] = interp_filter
    //             for(refList=0;refList<2;refList++) {
    //                 Mvs[r+y][c+x][refList] = BlockMvs[refList][3]
    //                 for(b=0;b<4;b++)
    //                     SubMvs[r+y][c+x][refList][b] = BlockMvs[refList][b]
    //             }
    //         } else {
    //             for(b=0;b<4;b++)
    //                 SubModes[r+y][c+x][b] = sub_modes[b]
    //         }
    //     }
}

// 6.4.5 Mode info syntax
#[allow(dead_code)]
fn mode_info() {
    // if (FrameIsIntra)
    //     intra_frame_mode_info()
    // else
    //     inter_frame_mode_info()
}

// 6.4.6 Intra frame mode info syntax
#[allow(dead_code)]
fn intra_frame_mode_info() {
    intra_segment_id()
    read_skip()
    read_tx_size(1)
    ref_frame[0] = INTRA_FRAME
    ref_frame[1] = NONE
    is_inter = 0
    if (MiSize >= BLOCK_8X8) {
        default_intra_mode # T
        y_mode = default_intra_mode
        for(b=0;b<4;b++)
            sub_modes[b] = y_mode
    } else {
        num4x4w = num_4x4_blocks_wide_lookup[MiSize]
        num4x4h = num_4x4_blocks_high_lookup[MiSize]
        for(idy=0;idy<2;idy+=num4x4h) {
            for(idx=0;idx<2;idx+=num4x4w) {
                default_intra_mode # T
                for(y2=0;y2<num4x4h;y2++)
                    for(x2=0;x2<num4x4w;x2++)
                        sub_modes[(idy+y2)*2+idx+x2] = default_intra_mode
            }
        }
        y_mode = default_intra_mode
    }
    default_uv_mode # T
    uv_mode = default_uv_mode
}

// 6.4.7 Intra segment id syntax
#[allow(dead_code)]
fn intra_segment_id() {
    // if (segmentation_enabled && segmentation_update_map)
    //     segment_id # T
    // else
    //     segment_id = 0
}

// 6.4.8 Skip syntax
#[allow(dead_code)]
fn read_skip() {
    // if (seg_feature_active(SEG_LVL_SKIP)) {
    //     skip = 1
    // } else {
    //     Skip # T
    // }
}

// 6.4.9 Segmentation feature active syntax
#[allow(dead_code)]
fn seg_feature_active(/* feature */) {
    // return segmentation_enabled && FeatureEnabled[segment_id][feature]
}

// 6.4.10 Tx size syntax
#[allow(dead_code)]
fn read_tx_size(/* allowSelect */) {
    // maxTxSize = max_txsize_lookup[MiSize]
    // if (allowSelect && tx_mode == TX_MODE_SELECT && MiSize >= BLOCK_8X8)
    //     tx_size # T
    // else
    //     tx_size = Min(maxTxSize, tx_mode_to_biggest_tx_size[tx_mode])
}

//
//
//
//
//
//
//
//
//
//
//
//
//
//
//
//
//
//
//
//

fn main() {
    // println!("Hello, world!");
}
