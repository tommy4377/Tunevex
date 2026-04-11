pub fn build_context_injection(
    profile_json: &str,
    applied_json: &str,
    memory_ctx: &str,
) -> (String, String) {
    let user = format!(
        r#"You are an expert Windows optimization assistant in TommyTweaker.

SYSTEM PROFILE:
{}

CURRENTLY APPLIED TWEAKS:
{}

YOUR PAST MEMORY (recent actions, diagnoses, outcomes — use this to avoid repeating mistakes):
{}

Rules:
- Always reference specific tweak IDs (e.g. `net_tcp_bbr`)
- If a diagnosis previously FAILED to resolve an issue, say so explicitly
- If you previously recommended a tweak and the user applied it, track its outcome
- Be honest about risks; do not recommend Dangerous tweaks unless explicitly asked
- If ram_gb is 0 the profiler failed — skip RAM-specific analysis
- Keep answers concise and technical"#,
        profile_json, applied_json, memory_ctx
    );
    let model = "Understood. I have full context of this system, its applied tweaks, \
         and my past memory. Ready to help."
        .to_string();
    (user, model)
}

pub fn build_analyze_prompt(profile_json: &str, tweaks_json: &str) -> String {
    format!(
        r#"You are a Windows optimization expert. Analyze this system EXHAUSTIVELY.

SYSTEM PROFILE:
{}

ALL AVAILABLE TWEAKS (sorted by category then id — evaluate every single one):
{}

INSTRUCTIONS — follow exactly, no exceptions:
1. Go through EVERY tweak in the list above one by one. Do not skip any.
2. For each tweak decide: ADD (not applied, beneficial for this hardware), REMOVE (applied but counterproductive), or ignore.
3. Only ADD tweaks genuinely useful for THIS specific CPU/GPU/RAM/connection.
4. Only REMOVE tweaks that are applied AND actively harmful (e.g. Intel tweak on AMD, laptop tweak on desktop, conflicting pair).
5. Only report CONFLICTS between tweaks that are BOTH currently_applied=true AND directly interfere.
6. Be deterministic: same system data must always produce the same output.
7. system_summary: 2-3 sentences referencing actual values (CPU model, RAM GB, GPU name).
8. If ram_gb is 0 the profiler failed — skip all RAM-specific recommendations.

Respond with ONLY valid JSON, no markdown fences, no extra text:
{{
  "system_summary": "string",
  "add":       [{{"id":"string","priority":"high|medium|low","reason":"one sentence with specific hardware data"}}],
  "remove":    [{{"id":"string","priority":"high|medium|low","reason":"why counterproductive on this exact hardware"}}],
  "conflicts": [{{"tweak_ids":["string","string"],"issue":"what the conflict causes"}}]
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
