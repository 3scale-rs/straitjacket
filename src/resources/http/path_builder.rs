use std::error::Error;

pub enum ParameterQuantifier {
    PairingSegments,
    JoiningSegments,
}

pub(super) struct PathBuilder<'a, 's> {
    segments: &'a [&'s str],
    quantifier: ParameterQuantifier,
}

// Untyped mixing of strings to build paths.
impl<'a, 's> PathBuilder<'a, 's> {
    pub const fn new(segments: &'a [&'s str], quantifier: ParameterQuantifier) -> Self {
        Self {
            segments,
            quantifier,
        }
    }

    pub fn accepted_parameters(&self) -> usize {
        match self.quantifier {
            ParameterQuantifier::PairingSegments => self.segments.len(),
            ParameterQuantifier::JoiningSegments => self.segments.len() - 1,
        }
    }

    // Similar to what itertools::Itertools::zip_longest would do
    pub fn build(&self, params: &[&str]) -> Result<String, Box<dyn Error>> {
        if params.len() != self.accepted_parameters() {
            return Err(From::from(format!(
                "required {} parameters but {} were provided",
                self.accepted_parameters(),
                params.len()
            )));
        }

        let mut s = self.segments.iter().zip(params.iter()).fold(
            String::new(),
            |mut acc, (segment, arg)| {
                acc.push_str(segment);
                acc.push_str(arg);
                acc
            },
        );

        // If we were just joining, the operation above would've stopped right before
        // appending the last segment, so append it now.
        if let ParameterQuantifier::JoiningSegments = self.quantifier {
            s.push_str(self.segments.last().unwrap());
        }

        Ok(s)
    }
}

#[cfg(test)]
mod tests {
    use super::ParameterQuantifier::*;
    use super::*;

    #[test]
    fn it_works_with_one_less_param_than_segments() {
        let pb = PathBuilder::new(&["/services/", "/metrics/", "/list"], JoiningSegments);
        let s = pb.build(&["123456", "abc"]);
        assert!(s.is_ok());
        let path = s.unwrap();
        assert_eq!(path, "/services/123456/metrics/abc/list");
    }

    #[test]
    fn it_works_with_same_params_as_segments() {
        let pb = PathBuilder::new(&["/services/", "/metrics/", "/list/"], PairingSegments);
        let s = pb.build(&["123456", "abc", "tail"]);
        assert!(s.is_ok());
        let path = s.unwrap();
        assert_eq!(path, "/services/123456/metrics/abc/list/tail");
    }

    #[test]
    fn it_fails_with_more_params_than_specified() {
        let pb = PathBuilder::new(&["/services/", "/metrics/", "/list/"], PairingSegments);
        let s = pb.build(&["123456", "abc", "tail", "extra"]);
        assert!(s.is_err());
    }

    #[test]
    fn it_fails_with_less_params_than_specified() {
        let pb = PathBuilder::new(&["/services/", "/metrics/", "/list/"], PairingSegments);
        let s = pb.build(&["123456", "abc"]);
        assert!(s.is_err());
    }
}
