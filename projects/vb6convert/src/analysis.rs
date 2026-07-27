/// Analysis and compatibility checking for VB6 projects
///
/// This module provides functionality to analyze VB6 projects and determine
/// conversion feasibility, identify potential issues, and estimate complexity.
use crate::error::Result;
use crate::traits::{Project, VB6Feature};
use crate::types::*;
use std::collections::{HashMap, HashSet};

/// Analyze a VB6 project for conversion compatibility
pub struct ProjectAnalyzer {
    detected_features: HashSet<VB6Feature>,
    complexity_score: f64,
    #[allow(dead_code)]
    warnings: Vec<ConversionWarning>,
}

impl ProjectAnalyzer {
    pub fn new() -> Self {
        Self {
            detected_features: HashSet::new(),
            complexity_score: 0.0,
            warnings: Vec::new(),
        }
    }

    /// Analyze a VB6 project
    pub fn analyze(&mut self, _project: &Project) -> Result<AnalysisReport> {
        // TODO: Implement actual analysis<'_>
        todo!("Project analysis not yet implemented")
    }

    /// Check if a feature is used in the project
    pub fn uses_feature(&self, feature: VB6Feature) -> bool {
        self.detected_features.contains(&feature)
    }

    /// Get the complexity score (0.0 = simple, 1.0 = very complex)
    pub fn complexity_score(&self) -> f64 {
        self.complexity_score
    }
}

impl Default for ProjectAnalyzer {
    fn default() -> Self {
        Self::new()
    }
}

/// Report from project analysis
#[derive(Debug, Clone)]
pub struct AnalysisReport {
    /// Features detected in the project
    pub features: HashSet<VB6Feature>,

    /// Complexity score
    pub complexity: f64,

    /// Statistics about the project
    pub stats: ProjectStats,

    /// Warnings and issues found
    pub warnings: Vec<ConversionWarning>,

    /// Recommendations for conversion
    pub recommendations: Vec<String>,
}

/// Statistics about a VB6 project
#[derive(Debug, Clone)]
pub struct ProjectStats {
    pub total_files: usize,
    pub modules: usize,
    pub classes: usize,
    pub forms: usize,
    pub total_lines: usize,
    pub api_calls: usize,
    pub database_connections: usize,
    pub third_party_controls: HashMap<String, usize>,
}
