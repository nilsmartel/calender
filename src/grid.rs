use std::marker::PhantomData;

use druid::*;
use druid::{Data, Lens};

pub struct Grid<F, D, FW, W>
where
    F: Fn(&D) -> (u32, u32),
    D: Data,
    FW: Fn(u32, u32) -> W,
    W: Widget<D>,
{
    make_widget: FW,
    get_size: F,
    widgets: Vec<Vec<W>>,
    _d: PhantomData<D>,
}

impl<F, D, FW, W> Grid<F, D, FW, W>
where
    F: Fn(&D) -> (u32, u32),
    D: Data,
    FW: Fn(u32, u32) -> W,
    W: Widget<D>,
{
    fn new(make_widget: FW, get_size: F) -> Self {
        Self {
            make_widget,
            widgets: Vec::new(),
            get_size,
            _d: PhantomData::default(),
        }
    }

    fn current_size(&self) -> (u32, u32) {
        let rows = self.widgets.len() as u32;
        let columns = if rows >= 1 {
            self.widgets[0].len() as u32
        } else {
            0
        };

        (columns, rows)
    }

    // Ensures the widget array has the appropriate dimension
    fn update_widgets(&mut self, data: &D) {
        let gs = &self.get_size;
        let (columns, rows) = gs(data);
        let (my_columns, my_rows) = self.current_size();

        // Ensure we don't have more rows than needed
        self.widgets.truncate(columns.min(my_columns) as usize);

        // Ensure we don't have more columns as needed
        for &mut w in self.widgets.iter_mut() {
            let len = w.len();
            w.truncate(len.min(rows as usize));
        }

        // include additional rows if needed
        for _ in self.widgets.len()..(rows as usize) {
            self.widgets.push(Vec::new());
        }

        // include additional columns if needed
        for (row_index, mut row) in self.widgets.iter_mut().enumerate() {
            for column_index in row.len()..(columns as usize) {
                let w = (self.make_widget)(column_index as u32, row_index as u32);
                row.push(w);
            }
        }
    }
}

impl<F, D, FW, W> Widget<D> for Grid<F, D, FW, W>
where
    F: Fn(&D) -> (u32, u32),
    D: Data,
    FW: Fn(u32, u32) -> W,
    W: Widget<D>,
{
    fn event(&mut self, ctx: &mut EventCtx, event: &Event, data: &mut D, env: &Env) {
        self.update_widgets(data);
        for row in self.widgets.iter_mut() {
            for child in row.iter_mut() {
                child.event(ctx, event, data, env);
            }
        }
    }

    fn lifecycle(&mut self, ctx: &mut LifeCycleCtx, event: &LifeCycle, data: &D, env: &Env) {
        self.update_widgets(data);
        for row in self.widgets.iter_mut() {
            for child in row.iter_mut() {
                child.lifecycle(ctx, event, data, env);
            }
        }
    }

    fn update(&mut self, ctx: &mut UpdateCtx, old_data: &D, data: &D, env: &Env) {
        self.update_widgets(data);
        for row in self.widgets.iter_mut() {
            for child in row.iter_mut() {
                child.update(ctx, old_data, data, env);
            }
        }
    }

    fn layout(&mut self, ctx: &mut LayoutCtx, bc: &BoxConstraints, _data: &D, _env: &Env) -> Size {
        bc.max()
    }

    fn paint(&mut self, ctx: &mut PaintCtx, data: &D, env: &Env) {
        self.update_widgets(data);

        let (columns, rows) = self.current_size();
        let Size { width, height } = ctx.size();

        // How much space each widget gets
        let offset_x = width / columns as f64;
        let offset_y = height / rows as f64;

        let mut pos_y = 0.0;
        for row in self.widgets.iter_mut() {
            let mut pos_x = 0.0;
            for child in row.iter_mut() {
                // TODO make child have different max bounds
                // and make it start as (pos_x, pos_y)
                child.paint(ctx, data, env);

                pos_x += offset_x;
            }

            pos_y += offset_y;
        }
    }
}
