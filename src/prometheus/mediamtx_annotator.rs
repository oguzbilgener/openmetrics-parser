use std::collections::HashMap;

pub fn annotate_mediamtx_metrics(file_bytes: &str) -> String {
  let mut result_builder: HashMap<&str, Vec<&str>> = HashMap::new();
  
  for line in file_bytes.lines() {
    if line.trim() == "" {
      continue;
    }
    
    let metric_name = line.split(&[' ', '{'][..]).next();
    match metric_name {
      Some(name) => {
        let rest = line.split(name).last().expect("Invalid metrics format");
        result_builder.entry(name).or_insert(Vec::new()).push(rest);
      }
      None => continue
    }
  }

  let mut result = String::from("# HELP paths The number of distinct paths");
  let total_paths = result_builder.get("paths");
  match total_paths {
    Some(paths) => {
      result.push_str("\n# TYPE paths gauge");
      result.push_str(&format!("\npaths {}", paths.len()));
    }
    None => ()
  }
  
  let total_rtsp_conns = result_builder.get("rtsp_conns");
  match total_rtsp_conns {
    Some(rtsp_conns) => {
      result.push_str("\n# TYPE rtsp_conns gauge");
      result.push_str(&format!("\nrtsp_conns {}", rtsp_conns.len()));
    }
    None => ()
  }

  let total_rtsp_sessions = result_builder.get("rtsp_sessions");
  match total_rtsp_sessions {
    Some(rtsp_sessions) => {
      result.push_str("\n# TYPE rtsp_sessions gauge");
      result.push_str(&format!("\nrtsp_sessions {}", rtsp_sessions.len()));
    }
    None => ()
  }

  let total_webrtc_sessions = result_builder.get("webrtc_sessions");
  match total_webrtc_sessions {
    Some(webrtc_sessions) => {
      result.push_str("\n# TYPE webrtc_sessions gauge");
      result.push_str(&format!("\nwebrtc_sessions {}", webrtc_sessions.len()));
    }
    None => ()
  }

  let total_hls_muxers = result_builder.get("hls_muxers");
  match total_hls_muxers {
    Some(hls_muxers) => {
      result.push_str("\n# TYPE hls_muxers gauge");
      result.push_str(&format!("\nhls_muxers {}", hls_muxers.len()));
    }
    None => ()
  }

  let total_rtsps_conns = result_builder.get("rtsps_conns");
  match total_rtsps_conns {
    Some(rtsps_conns) => {
      result.push_str("\n# TYPE rtsps_conns gauge");
      result.push_str(&format!("\nrtsps_conns {}", rtsps_conns.len()));
    }
    None => ()
  }

  let total_rtmp_conns = result_builder.get("rtmp_conns");
  match total_rtmp_conns {
    Some(rtmp_conns) => {
      result.push_str("\n# TYPE rtmp_conns gauge");
      result.push_str(&format!("\nrtmp_conns {}", rtmp_conns.len()));
    }
    None => ()
  }

  let total_rtmps_conns = result_builder.get("rtmp_conns");
  match total_rtmps_conns {
    Some(rtmps_conns) => {
      result.push_str("\n# TYPE rtmps_conns gauge");
      result.push_str(&format!("\nrtmps_conns {}", rtmps_conns.len()));
    }
    None => ()
  }

  let total_srt_conns = result_builder.get("srt_conns");
  match total_srt_conns {
    Some(srt_conns) => {
      result.push_str("\n# TYPE srt_conns gauge");
      result.push_str(&format!("\nsrt_conns {}", srt_conns.len()));
    }
    None => ()
  }

  result_builder.remove("paths");
  result_builder.remove("rtsp_conns");
  result_builder.remove("rtsp_sessions");
  result_builder.remove("webrtc_sessions");
  result_builder.remove("hls_muxers");

  for key in result_builder.keys() {
    result.push_str(&format!("\n# TYPE {} counter", key));
    for metric in result_builder.get(key).expect("Forced") {
      result.push_str(&format!("\n{}{}", key, metric));
    }
  }

  result.push_str("\n# EOF");
  result
}
