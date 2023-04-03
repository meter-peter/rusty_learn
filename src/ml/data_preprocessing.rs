pub struct MinMaxScaler {
    min: Vec<f64>,
    max: Vec<f64>,
}

impl MinMaxScaler {
    pub fn new() -> Self {
        MinMaxScaler { min: vec![], max: vec![] }
    }

    pub fn fit(&mut self, data: &[Vec<f64>]) {
        let n_features = data[0].len();
        self.min = vec![std::f64::INFINITY; n_features];
        self.max = vec![std::f64::NEG_INFINITY; n_features];

        for row in data {
            for (i, &value) in row.iter().enumerate() {
                self.min[i] = self.min[i].min(value);
                self.max[i] = self.max[i].max(value);
            }
        }
    }

    pub fn transform(&self, data: &[Vec<f64>]) -> Vec<Vec<f64>> {
        let n_features = data[0].len();
        let mut transformed = data.to_owned();

        for row in &mut transformed {
            for i in 0..n_features {
                let range = self.max[i] - self.min[i];
                row[i] = (row[i] - self.min[i]) / range;
            }
        }

        transformed
    }

    pub fn fit_transform(&mut self, data: &[Vec<f64>]) -> Vec<Vec<f64>> {
        self.fit(data);
        self.transform(data)
    }
}

pub struct MeanNormalizer {
    mean: Vec<f64>,
}

impl MeanNormalizer {
    pub fn new() -> Self {
        MeanNormalizer { mean: vec![] }
    }

    pub fn fit(&mut self, data: &[Vec<f64>]) {
        let n_features = data[0].len();
        let n_samples = data.len();
        self.mean = vec![0.0; n_features];

        for row in data {
            for (i, &value) in row.iter().enumerate() {
                self.mean[i] += value;
            }
        }

        for i in 0..n_features {
            self.mean[i] /= n_samples as f64;
        }
    }

    pub fn transform(&self, data: &[Vec<f64>]) -> Vec<Vec<f64>> {
        let n_features = data[0].len();
        let mut transformed = data.to_owned();

        for row in &mut transformed {
            for i in 0..n_features {
                row[i] -= self.mean[i];
            }
        }

        transformed
    }

    pub fn fit_transform(&mut self, data: &[Vec<f64>]) -> Vec<Vec<f64>> {
        self.fit(data);
        self.transform(data)
    }
}

pub fn one_hot_encode(data: &[usize], n_classes: usize) -> Vec<Vec<u8>> {
    let n_samples = data.len();
    let mut one_hot_encoded = vec![vec![0u8; n_classes]; n_samples];

    for (i, &class) in data.iter().enumerate() {
        one_hot_encoded[i][class] = 1;
    }

    one_hot_encoded
}