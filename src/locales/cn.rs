use std::collections::HashMap;

pub fn get_messages() -> HashMap<&'static str, &'static str> {
    let mut messages = HashMap::new();

    // 错误消息
    messages.insert("error_clone_failed", "删除克隆目录失败");
    messages.insert("cloning_repo", "正在克隆GitHub仓库");
    messages.insert("analysis_target", "分析目标");
    messages.insert("context_collection_failed", "收集上下文失败");
    messages.insert("analyzing_file", "正在分析文件");
    messages.insert("analysis_completed", "分析完成");
    messages.insert("error_directory_creation", "创建目录失败");
    messages.insert("error_no_write_permission", "无写入权限");
    messages.insert("error_test_file_deletion", "删除测试文件失败");
    messages.insert(
        "error_no_file_creation_permission",
        "无文件创建权限",
    );
    messages.insert(
        "error_output_dir_check",
        "❌ 检查输出目录失败",
    );
    messages.insert("relevant_files_detected", "检测到相关源文件");
    messages.insert(
        "security_pattern_files_detected",
        "检测到安全模式匹配文件",
    );
    messages.insert("parse_add_failed", "添加文件到解析器失败");
    messages.insert("analysis_failed", "分析失败");
    messages.insert(
        "markdown_report_output_failed",
        "输出Markdown报告失败",
    );
    messages.insert("markdown_report_output", "输出Markdown报告");
    messages.insert(
        "summary_report_output_failed",
        "输出摘要报告失败",
    );
    messages.insert("summary_report_output", "输出摘要报告");
    messages.insert(
        "summary_report_needs_output_dir",
        "摘要报告输出需要--output-dir选项",
    );
    messages.insert(
        "sarif_report_output_failed",
        "输出SARIF报告失败",
    );
    messages.insert("sarif_report_output", "输出SARIF报告");
    messages.insert("sarif_output_failed", "输出SARIF失败");
    messages.insert(
        "github_repo_clone_failed",
        "克隆GitHub仓库失败",
    );
    messages.insert(
        "custom_pattern_generation_start",
        "开始自定义模式生成模式",
    );
    messages.insert(
        "pattern_generation_completed",
        "模式生成完成",
    );

    messages
}

pub const SYS_PROMPT_TEMPLATE: &str = r#"
作为安全研究员，分析代码漏洞时需特别注意：
- 输入验证和清理
- 认证和授权
- 数据处理和泄漏
- 命令注入可能性
- 路径遍历漏洞
- 时序攻击和竞态条件
- 其他安全关键模式
"#;

pub const INITIAL_ANALYSIS_PROMPT_TEMPLATE: &str = r#"
基于PAR(Principal-Action-Resource)模型分析给定代码，并以以下JSON格式输出结果：

**主体(谁/数据源)**: 作为数据起源的实体或输入源
- 用户输入、API响应、文件读取、环境变量等
- 评估每个主体的信任级别(trusted/semi_trusted/untrusted)

**动作(什么/验证-处理)**: 数据处理或验证操作(包括漏洞可能性)
- 输入验证、清理、认证/授权、加密等(注意绕过可能性)
- 评估实现质量(adequate/insufficient/missing/bypassed)

**资源(哪里/危险操作)**: 影响机密性、完整性和可用性的操作
- 文件写入、命令执行、数据库更新、输出等
- 评估机密性级别(low/medium/high/critical)

评估每个代码元素的PAR角色，是否实现了适当的安全策略，并报告为policy_violations。
"#;

pub const ANALYSIS_APPROACH_TEMPLATE: &str = r#"
基于PAR模型的分析步骤：
1. **主体识别**: 识别危险数据源(不可信输入)
2. **资源识别**: 识别影响机密性、完整性和可用性的危险操作
3. **动作评估**: 评估在从主体到资源的路径上是否实现了适当的验证和防御措施
4. **策略违规检测**: 检测危险主体不经过适当动作直接访问资源的情况
5. **PAR关系上下文评估**: 在整个代码上下文中判断PAR关系是否恰当
"#;

pub const GUIDELINES_TEMPLATE: &str = r#"
基于PAR的安全策略评估指南：

## 分析步骤
1. **主体评估**: 识别不可信数据源并评估其风险
2. **资源评估**: 评估影响机密性、完整性和可用性的操作风险
3. **动作评估**: 评估主体和资源之间是否实现了适当的防御措施
4. **策略违规**: 检测危险主体不经过适当动作直接访问资源的情况
5. **上下文考虑**: 在整个代码上下文中判断PAR关系是否恰当
6. **声明式判断**: 使用"此主体可以执行此动作"等声明式策略进行评估
7. 请用中文回答

重要：
- 如果没有漏洞存在，返回confidence_score=0，vulnerability_types=[]，以及空的par_analysis。
- 动作模式(验证/处理)有绕过可能性，实现缺陷是漏洞的直接原因。
"#;

pub const EVALUATOR_PROMPT_TEMPLATE: &str = r#"你是一名安全专家，正在评估漏洞分析报告。
该报告旨在识别一个已知包含SQL注入(SQLI)、跨站脚本(XSS)和远程代码执行(RCE)漏洞的Python Web应用中的漏洞。

请从以下角度评估报告：
1. 正确识别的漏洞(SQLI、XSS、RCE)
2. 误报(报告了不存在的漏洞)
3. 分析质量(影响评估、根本原因解释、缓解建议)
4. 验证代码质量(清晰步骤、示例请求、预期结果)

待评估的报告：
{report}
"#;

pub const RESPONSE_LANGUAGE_INSTRUCTION: &str = "请用中文回答";