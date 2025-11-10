//! This is a debugging tool for [`XLayout`](super::XLayout) that provides a record of
//! how an area was split into segments.
use super::{Debug, Display, Formatter, STEP_FACTOR, SegmentRule, SegmentTarget, Vec};

/// An optional [`FillRecord`] that may record the steps taken in the layout algorithm,
/// if the algorithm is being run for debugging. Otherwise this will be `None` and
/// nothing will be recorded.
pub struct OptionFillRecord(pub Option<FillRecord>);

impl OptionFillRecord {
    /// Populate the list of segments in the record from a [`SegmentRule`] array.
    pub fn set_segments(&mut self, area_size: i32, segments: &[SegmentRule]) {
        if let Some(r) = &mut self.0 {
            r.area_size = area_size;
            r.segments = segments.iter().map(SegmentRule::record).collect();
        }
    }
    /// Reset the record so that the layout process can restart with the same
    /// segments and an empty list of steps.
    pub fn reset(&mut self) {
        if let Some(r) = &mut self.0 {
            r.steps.clear();
        }
    }
    /// Add a step to the record.
    pub fn add_step(
        &mut self,
        index: usize,
        before: i32,
        after: i32,
        size: u64,
        target: SegmentTarget,
    ) {
        if let Some(r) = &mut self.0 {
            r.steps.push(FillRecordStep {
                index,
                before,
                after,
                size,
                target,
            });
        }
    }
}

#[allow(clippy::doc_markdown)]
/// A record of the steps used by a `XLayout` to split an area into segments. It contains a list of
/// segments, one for each constraint in order, even including the constraints with the
/// [`is_separator`](super::XConstraint::is_separator) flag. The `Display` implementation concisely
/// formats the data to visualize the layout process.
///
/// For example, this layout operation would produce something like the following result.
///
/// ```
/// use ratatui_core::layout::{IntoConstraint, Rect, XConstraint, XLayout};
/// let layout = XLayout::vertical(
///     (3..=10).scale(1).list() + XConstraint::overlap(8).scale(0) + (5..=12).scale(2),
/// );
/// let area = Rect::new(0, 0, 20, 15);
/// println!("{}", layout.split_for_debug(area));
/// ```
///
/// Output:
///
/// ```text
/// area size: 15 (min/pref/max)
/// [0: 0/3/10 scale:1 priority:0, 1: -8/-8/0 scale:0 priority:100, 2: 0/5/12 scale:2 priority:0]
/// step  0: [  0]  0 -> 3   +3 7+22/32 Preferred (1 * 3 + 0) 0/3/10 scale:1 priority:0
/// step  1: [  2]  0 -> 5   +5 7+22/32 Preferred (2 * 2 + 1) 0/5/12 scale:2 priority:0
/// step  2: [  0]  3 -> 8   +5 5+0/32 Max (1 * 5 + 0) 0/3/10 scale:1 priority:0
/// step  3: [  2]  5 -> 12  +7 5+0/32 Max (2 * 3 + 1) 0/5/12 scale:2 priority:0
/// step  4: [  0]  8 -> 10  +2 3+0/32 Max (1 * 2 + 0) 0/3/10 scale:1 priority:0
/// step  5: [  0] 10 -> 11  +1 0+11/32 Forced (1 * 1 + 0) 0/3/10 scale:1 priority:0
/// final: [11, -8, 12]
/// ```
///
/// The first line tells us length of the area in the layout direction: 15, and it reminds us of the
/// format in which ranges are expressed, first min, then preferred, then max, separated by `/`.
///
/// Next we get a list of the segments that will be allocating. In this case there are three
/// segments, one for each constraint. In this case
/// [`XConstraint::overlap`](super::XConstraint::overlap) returns a separator constraint, so its
/// segment would not be returned from a call to [`XLayout::split`](super::XLayout::split)
/// but the layout algorithm does not care about that at this stage.
///
/// Next we have a list of 6 steps that were require to split this area. Each line is one step, and
/// they are formatted like this:
///
/// ```text
/// {step #}{index}{before}{after}{change}{step size}{phase}{amount used}{targets}{scale}{priority}
/// step  0: [  0]     0  ->   3     +3    7+22/32 Preferred (1 * 3 + 0) 0/3/10 scale:1 priority:0
/// ```
///
/// 1. `step #`: The number of steps that came before this step.
/// 2. `index`: The index of the constraint that controls this segment in the constraint's array,
///    and the index of the segment in the above list of segments.
/// 3. `before`: The amount allocated to this segment before this step.
/// 4. `after`: The amount allocated to this segment after this step.
/// 5. `change`: `after - before`
/// 6. `step size`: See [`FillRecordStep::size`]. This represents how much the constraint is
///    permitted to allocate in this step. Here the number is divided by 32 to make it easier to
///    read. Multiply that by the segment's scale to get the maximum possible allocation. Every
///    active constraint is given the same step size for one step, so having a larger scale means a
///    larger potential for allocation each step.
/// 7. `phase`: This indicates which target the segment is growing toward. In this case, it is
///    growing toward its preferred size, which is 3.
/// 8. `amount used`: This indicates how much of the step size was actually used. `1 * 3 + 0` means
///    that the scale was 1, and that was multiplied by 3 (out of a maximum of 7) from the step
///    size, plus 0 additional size from using less than a full scale. If the segment had used its
///    full step size, this would have said `1 * 7 + 1`. The + 1 comes from 22/32 rounded up. It
///    could not do that because 3 reached the target for this step.
/// 9. `targets`: `0/3/10`. These are the three targets, the minimum, the preferred, and the
///    maximum. In this case the phase is `Preferred` so the allocation target is 3. The target for
///    the next phase will be 10.
/// 10. `scale`: This is the factor that gets multiplied by the step size to determine how much the
///     segment is allowed to allocate on this step. See
///     [`HintRange::fill_scale`](super::HintRange::fill_scale).
/// 11. `priority`: In this example all the segments have the same priority, so all of them are
///     immediately active and stay active until they reach their target. If there were multiple
///     priorities, then segments with earlier priority would activate first and take steps until
///     they reach their current target, and only after all the active segments are satisfied would
///     segments from the next priority become active.
///
/// In step 0 and 1, segments 0 and 2 are each allowed a step size of 7+22/32, and they both
/// immediately use that to reach their preferred targets. Segment 1 does not get a step because its
/// scale is 0. The reason the step size is 7+22/32 is because the layout has 23 space to fill,
/// 15 from the width of the area, +8 due to segment 1 having negative size and causing the other
/// segments to overlap. If both segments 0 and 2 used their full 7+22/32, then segment 0 would
/// allocate 8 and segment 2 would allocate 15 (2 * 7+22/32), which is a total of 23. They are being
/// given the maximum step size that is safe to take without a chance of overflowing the area.
///
/// In step 2 and 3, segments 0 and 2 get a step size of 5. The step size is smaller because less of
/// the area is available after the first round. Segment 2 uses 3 of that 5 to immediately bring
/// itself to its target preferred size of 12 by multiplying 3 by its scale of 2. Unfortunately, the
/// step size is insufficient for segment 0, so it uses the full step size and only reaches 8, which
/// is less than its target of 10.
///
/// In step 4, the only remaining active segment is segment 0, so it is given a step size of 3,
/// which authorizes it to allocate the entire remaining area. In order to reach its target, it only
/// uses 2. This leaves 1 space remaining in the area that has not been allocated.
///
/// The max phase and the overfill phase are skipped, because segments 0 and 2 have both already
/// reached their max target, and neither has the
/// [`HintRange::overfill`](super::HintRange::overfill) flag set. Therefore step 5 skips straight to
/// the forced phase where all segments must grow regardless of targets until the entire area is
/// allocated. Segment 1 is exempt because having a scale of 0 means it still cannot grow. Segment 0
/// takes the first step and is given a step size of 11/32 because there is only 1 unit of space to
/// fill. In this case the small step size does not matter, because every segment is always allowed
/// to allocate at least one unit on each step, so segment 0 takes that unit and the layout is
/// complete.
///
/// The final line shows the result: `[11, -8, 12]`. Segment 0 was forced to exceed its maximum by 1
/// to reach 11, segment 1 started at -8 and remained there to the end because its scale was 0, and
/// segment 2 reached its preferred size of 12.
#[derive(Debug, Default, Clone)]
pub struct FillRecord {
    /// The size of the area to layout along the layout axis.
    pub area_size: i32,
    /// The list of segments, each one controlled by a constraint.
    pub segments: Vec<FillRecordSegment>,
    /// The list of steps in the layout process, each one allocating a certain amount of space
    /// to a certain segment.
    pub steps: Vec<FillRecordStep>,
}

impl Display for FillRecord {
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        write!(f, "area size: {} (min/pref/max)\n[", self.area_size)?;
        let mut iter = self.segments.iter().enumerate();
        if let Some((i, seg)) = iter.next() {
            write!(f, "{i}: {seg}")?;
        }
        for (i, seg) in iter {
            write!(f, ", {i}: {seg}")?;
        }
        f.write_str("]\n")?;
        let mut results = self
            .segments
            .iter()
            .map(FillRecordSegment::initial_size)
            .collect::<Vec<_>>();
        for (i, step) in self.steps.iter().enumerate() {
            let amount = step.after.saturating_sub(step.before);
            let Some(segment) = self.segments.get(step.index) else {
                writeln!(f, "step{i:3}: [{:3}] Invalid index", step.index)?;
                continue;
            };
            let scale = segment.fill_scale;
            let steps = amount.checked_div(scale).unwrap_or(0);
            let rem = amount.checked_rem(scale).unwrap_or(amount);
            writeln!(f, "step{i:3}: {step} ({scale} * {steps} + {rem}) {segment}")?;
            if let Some(v) = results.get_mut(step.index) {
                *v = step.after;
            } else {
                f.write_str("Invalid index\n")?;
            }
        }
        write!(f, "final: {results:?}")
    }
}

/// The values that are used to allocate space to a segment during layout, after the constraints
/// have calculated absolute values given the area that needs to be filled.
#[derive(Debug, Clone)]
pub struct FillRecordSegment {
    /// The target size for this segment during the min phase of layout.
    /// See [`HintRange::min`](super::HintRange::min).
    pub min: i32,
    /// The target size for this segment during the preferred phase of layout.
    /// See [`HintRange::preferred`](super::HintRange::preferred).
    pub preferred: i32,
    /// The target size for this segment during the max phase of layout.
    /// See [`HintRange::max`](super::HintRange::max).
    pub max: i32,
    /// The speed at which this segment grows relative to other segments.
    /// See [`HintRange::fill_scale`](super::HintRange::fill_scale).
    pub fill_scale: i32,
    /// True if this segment will allocate during the [`SegmentTarget::Overfill`] phase.
    pub overfill: bool,
    /// The order in which segments are allowed to allocate size.
    /// Lower values allocate first during the min and preferred phase,
    /// then higher values allocate first during later phases.
    /// See [`HintRange::priority`](super::HintRange::priority).
    pub priority: i16,
}

impl Display for FillRecordSegment {
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        write!(
            f,
            "{}/{}/{} scale:{}",
            self.min, self.preferred, self.max, self.fill_scale
        )?;
        if self.overfill {
            f.write_str(" overfill")?;
        }
        write!(f, " priority:{}", self.priority)
    }
}

impl FillRecordSegment {
    /// The size of the segment when layout begins, based upon its
    /// min size and fill scale. Most segments begin at zero size, but
    /// a negative min size or a fill scale of zero will cause the min size
    /// to be used instead of zero.
    pub fn initial_size(&self) -> i32 {
        if self.fill_scale == 0 {
            self.min
        } else {
            self.min.min(0)
        }
    }
}

/// A step in the layout process, allocating a certain amouht of space to a certain segment.
#[derive(Debug, Clone)]
pub struct FillRecordStep {
    /// The index of the segment in the segment list, and also the index of the constraint
    /// in the constraint list.
    pub index: usize,
    /// The size of the segment before space was allocated to it in this step.
    pub before: i32,
    /// The size of the segment after the step.
    pub after: i32,
    /// The size of the step. Each time the layout takes a step, it looks through
    /// all the active constraints and how fast each one wants to allocate based on its
    /// [`HintRange::fill_scale`](super::HintRange::fill_scale), and then checks the remaining
    /// available space to determine how large a step each constraint should make in allocating
    /// space while still giving all the other active constraints their chance.
    ///
    /// The amount that a constraint is allowed to allocate for itself in this step is
    /// `fill_scale * size / 32`
    ///
    /// If the amount allocated is less than this, it is because the constraint reached its target
    /// and has become inactive.
    pub size: u64,
    /// The target of the current step, indicating whether the constraints are trying to reach
    /// their min, preferred, or max sizes.
    pub target: SegmentTarget,
}

impl Display for FillRecordStep {
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        let index = self.index;
        let before = self.before;
        let after = self.after;
        let amount = after.saturating_sub(before);
        let whole_steps = self.size / STEP_FACTOR;
        let part_steps = self.size % STEP_FACTOR;
        let target = self.target;
        write!(
            f,
            "[{index:3}]{before:3} -> {after:<3}{amount:+3} {whole_steps}+{part_steps}/{STEP_FACTOR} {target}",
        )
    }
}

#[cfg(test)]
mod tests {
    use alloc::string::ToString;

    use crate::layout::{IntoConstraint, Rect, XConstraint, XLayout};

    #[test]
    fn layout_debug1() {
        const EXPECTED: &str = "area size: 8 (min/pref/max)\n\
        [0: 8/8/65535 scale:1 priority:0, 1: 2/2/65535 scale:0 priority:0, 2: 8/8/65535 scale:2 priority:0]\n\
        step  0: [  0]  0 -> 2   +2 2+0/32 Min (1 * 2 + 0) 8/8/65535 scale:1 priority:0\n\
        step  1: [  2]  0 -> 4   +4 2+0/32 Min (2 * 2 + 0) 8/8/65535 scale:2 priority:0\n\
        final: [2, 2, 4]";
        let layout = XLayout::vertical(
            (.., 8).scale(1).list() + (.., 2).separator().scale(0) + (.., 8).scale(2),
        );
        let area = Rect::new(0, 0, 20, 8);
        assert_eq!(&layout.split_for_debug(area).to_string(), EXPECTED);
    }
    #[test]
    fn layout_debug2() {
        const EXPECTED: &str = "area size: 15 (min/pref/max)\n\
        [0: 0/3/10 scale:1 priority:0, 1: -8/-8/0 scale:0 priority:100, 2: 0/5/12 scale:2 priority:0]\n\
        step  0: [  0]  0 -> 3   +3 7+22/32 Preferred (1 * 3 + 0) 0/3/10 scale:1 priority:0\n\
        step  1: [  2]  0 -> 5   +5 7+22/32 Preferred (2 * 2 + 1) 0/5/12 scale:2 priority:0\n\
        step  2: [  0]  3 -> 8   +5 5+0/32 Max (1 * 5 + 0) 0/3/10 scale:1 priority:0\n\
        step  3: [  2]  5 -> 12  +7 5+0/32 Max (2 * 3 + 1) 0/5/12 scale:2 priority:0\n\
        step  4: [  0]  8 -> 10  +2 3+0/32 Max (1 * 2 + 0) 0/3/10 scale:1 priority:0\n\
        step  5: [  0] 10 -> 11  +1 0+11/32 Forced (1 * 1 + 0) 0/3/10 scale:1 priority:0\n\
        final: [11, -8, 12]";
        let layout = XLayout::vertical(
            (0..=10).preferred(3).scale(1).list()
                + XConstraint::overlap(8).scale(0)
                + (0..=12).preferred(5).scale(2),
        );
        let area = Rect::new(0, 0, 20, 15);
        assert_eq!(&layout.split_for_debug(area).to_string(), EXPECTED);
    }
}
