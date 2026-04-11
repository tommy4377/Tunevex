pub fn build_context_injection(profile_json: &str, applied_json: &str) -> (String, String) {
    let user = format!(
        r#"You are an expert Windows optimization assistant embedded in TommyTweaker,
a professional system tweaking app built with Tauri and Rust.

You have full access to the user's system state:

SYSTEM PROFILE:
{}

CURRENTLY APPLIED TWEAKS:
{}

Rules:
- Always reference specific tweak IDs when discussing them (e.g. "net_tcp_ack_freq")
- Be honest about risks — do not recommend Dangerous tweaks unless explicitly asked
- If something could cause instability, say so clearly
- Keep answers concise and technical"#,
        profile_json, applied_json
    );
    let model =
        "Understood. I have full context of this system and its applied tweaks. Ready to help."
            .to_string();
    (user, model)
}

pub fn build_analyze_prompt(profile_json: &str, tweaks_json: &str) -> String {
    format!(
        r#"Analyze this Windows system and return optimization recommendations.

SYSTEM PROFILE:
{}

ALL AVAILABLE TWEAKS (each has: id, name, description, risk_level, category, currently_applied):
{}

Your task:
1. Identify tweaks to ADD — not yet applied, would genuinely benefit THIS specific system
2. Identify tweaks to REMOVE — currently applied but useless or harmful for this system
   (e.g. Intel-specific tweak on an AMD system, laptop tweak on a desktop, etc.)
3. Identify conflicts between currently applied tweaks
4. Write a 2-3 sentence honest assessment of the system's current state

Respond with ONLY a JSON object — no markdown code fences, no explanations. No array wrapper.
{{
  "system_summary": "string",
  "add": [
    {{
      "id": "string",
      "priority": "high|medium|low",
      "reason": "one sentence referencing actual system data"
    }}
  ],
  "remove": [
    {{
      "id": "string",
      "priority": "high|medium|low",
      "reason": "why it is counterproductive on this system"
    }}
  ],
  "conflicts": [
    {{
      "tweak_ids": ["string"],
      "issue": "what the conflict causes"
    }}
  ]
}}"#,
        profile_json, tweaks_json
    )
}

pub fn build_diagnose_prompt(
    profile_json: &str,
    applied_tweaks_json: &str,
    problem: &str,
) -> String {
    format!(
        r#"A user is experiencing a problem with their Windows PC.

PROBLEM DESCRIPTION: {}

SYSTEM PROFILE:
{}

CURRENTLY APPLIED TWEAKS (these are the only changes made to the system):
{}

Analyze which applied tweaks could be causing or contributing to this problem.
Consider interactions between tweaks, not just individual ones.

Respond with ONLY a JSON object — no markdown code fences, no explanations.
{{
  "likely_causes": [
    {{
      "tweak_id": "string",
      "confidence": "high|medium|low",
      "explanation": "specific reason this tweak causes the described symptom"
    }}
  ],
  "suggested_fix": "clear step-by-step instructions to resolve the issue",
  "safe_to_revert": ["tweak_id_1", "tweak_id_2"]
}}"#,
        problem, profile_json, applied_tweaks_json
    )
}
