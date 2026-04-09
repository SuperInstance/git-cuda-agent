//! ML feedback loop — runtime model improvement

/// Experience entry for learning
#[derive(Debug, Clone)]
pub struct Experience {
    pub query: String,
    pub response: String,
    pub score: f64,       // 0-1 quality score
    pub confidence: f64,
    pub timestamp_us: u64,
}

/// Feedback statistics
#[derive(Debug, Default)]
pub struct FeedbackStats {
    pub total_experiences: usize,
    pub avg_score: f64,
    pub improvement_rate: f64,
    pub best_score: f64,
    pub worst_score: f64,
}

/// ML feedback loop
pub struct FeedbackLoop {
    pub experiences: Vec<Experience>,
    pub window_size: usize,
    pub learning_rate: f64,
}

impl FeedbackLoop {
    pub fn new(window_size: usize, learning_rate: f64) -> Self {
        FeedbackLoop { experiences: Vec::new(), window_size, learning_rate }
    }

    /// Record an experience
    pub fn record(&mut self, query: &str, response: &str, score: f64, confidence: f64) {
        self.experiences.push(Experience {
            query: query.to_string(), response: response.to_string(),
            score, confidence, timestamp_us: 0,
        });
        if self.experiences.len() > self.window_size {
            self.experiences.remove(0);
        }
    }

    /// Get improvement statistics
    pub fn stats(&self) -> FeedbackStats {
        if self.experiences.is_empty() { return FeedbackStats::default(); }
        let scores: Vec<f64> = self.experiences.iter().map(|e| e.score).collect();
        let avg = scores.iter().sum::<f64>() / scores.len() as f64;
        let best = scores.iter().cloned().fold(f64::NEG_INFINITY, f64::max);
        let worst = scores.iter().cloned().fold(f64::INFINITY, f64::min);
        let improvement = if scores.len() >= 2 {
            let first_half: f64 = scores[..scores.len()/2].iter().sum::<f64>() / (scores.len()/2) as f64;
            let second_half: f64 = scores[scores.len()/2..].iter().sum::<f64>() / (scores.len() - scores.len()/2) as f64;
            (second_half - first_half) / first_half.max(0.001)
        } else { 0.0 };
        FeedbackStats { total_experiences: self.experiences.len(), avg_score: avg,
            improvement_rate: improvement, best_score: best, worst_score: worst }
    }

    /// Suggest confidence adjustment based on recent performance
    pub fn suggest_confidence(&self, base: f64) -> f64 {
        let stats = self.stats();
        let adjustment = (stats.avg_score - 0.5) * self.learning_rate;
        (base + adjustment).clamp(0.1, 1.0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_feedback_loop() {
        let mut loop_ = FeedbackLoop::new(100, 0.01);
        for i in 0..20 {
            let score = 0.5 + (i as f64) * 0.02; // improving
            loop_.record("q", "r", score, 0.9);
        }
        let stats = loop_.stats();
        assert!(stats.improvement_rate > 0.0, "Should show improvement");
        println!("Feedback: avg={:.2}, improvement={:.2}%", stats.avg_score, stats.improvement_rate * 100.0);
    }
}
