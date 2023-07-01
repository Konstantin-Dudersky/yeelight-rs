pub fn limit<T>(min: T, value: T, max: T) -> T
where
    T: PartialOrd,
{
    if value < min {
        return min;
    } else if value > max {
        return max;
    }
    value
}
