//
// This is only a SKELETON file for the 'Saddle Points' exercise. It's been provided as a
// convenience to get you started writing code faster.
//

export const saddlePoints = (matrix) => {
  if (matrix.length === 0) return [];

  const rows = matrix.length;
  const cols = matrix[0].length;

  // Precompute max of each row
  const rowMax = matrix.map(row => Math.max(...row));

  // Precompute min of each column
  const colMin = Array.from({ length: cols }, (_, col) =>
    Math.min(...matrix.map(row => row[col]))
  );

  const result = [];

  for (let i = 0; i < rows; i++) {
    for (let j = 0; j < cols; j++) {
      if (matrix[i][j] === rowMax[i] && matrix[i][j] === colMin[j]) {
        result.push({ row: i + 1, column: j + 1 }); // 1-based indexing
      }
    }
  }

  return result;
};

