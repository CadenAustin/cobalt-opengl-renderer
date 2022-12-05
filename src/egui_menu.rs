use std::{path::PathBuf, error::Error};

use egui::{collapsing_header::CollapsingState, Checkbox, CollapsingHeader, Ui};
use native_dialog::FileDialog;

use crate::{
    scene::{MeshDesc, ObjDesc},
    stage::Stage,
};

pub enum TreeReturn {
    DeleteMesh(usize, usize),
    DeleteObj(usize),
    SetTarget(MeshDesc)
}

pub fn draw_object_tree(ui: &mut Ui, objs: &mut Vec<ObjDesc>) -> Option<TreeReturn> {
    ui.label("Mesh Tree:");
    let mut select = false;
    let mut tree_ret = None;
    for (obj_idx, obj) in objs.iter_mut().enumerate() {
        CollapsingHeader::new(&obj.name).show(ui, |ui| {
            if ui.button("Delete OBJ").clicked() { tree_ret = Some(TreeReturn::DeleteObj(obj_idx)) };
            for (mesh_idx, mesh) in &mut obj.meshes.iter_mut().enumerate() {
                let name_id = ui.make_persistent_id(format!("{}-{}", mesh.name, mesh.mesh_id));
                CollapsingState::load_with_default_open(ui.ctx(), name_id, false)
                    .show_header(ui, |ui| {
                        ui.label(&mesh.name);
                        if ui.toggle_value(&mut select, "Look at Mesh").changed() {
                            tree_ret = Some(TreeReturn::SetTarget(mesh.clone()));
                        }
                    })
                    .body(|ui| {
                        if ui.button("Delete Mesh").clicked() { tree_ret = Some(TreeReturn::DeleteMesh(obj_idx, mesh_idx)) };
                        ui.checkbox(&mut mesh.show, "Show");
                        ui.checkbox(&mut mesh.highlight, "Highlight");
                    });
            }
        });
    }

    tree_ret
}

pub fn import_object(ui: &mut Ui) -> Option<PathBuf> {
    ui.label("Import Object:");
    if (ui.button("Add new object from obj file").clicked()) {
        let path = FileDialog::new()
            .add_filter("OBJ File", &["obj"])
            .show_open_single_file()
            .unwrap();
        return path;
    };

    None
}

pub fn draw_egui(stage: &mut Stage, ctx: &mut miniquad::Context) {
    let mut tree_ret = None;
    let mut new_obj_fp: Option<PathBuf> = None;
    stage.egui_mq.run(ctx, |_mq_ctx, egui_ctx| {
        egui::Window::new("cobalt options").show(egui_ctx, |ui| {
            tree_ret = draw_object_tree(ui, &mut stage.objs);
            new_obj_fp = import_object(ui);
        });
    });

    

    if let Some(ret) = tree_ret {
        let result: Result<(), Box<dyn Error>> = match ret {
            TreeReturn::DeleteMesh(obj_idx, mesh_idx) => {stage.objs[obj_idx].meshes.remove(mesh_idx); Ok(())},
            TreeReturn::DeleteObj(obj_idx) => {stage.objs.remove(obj_idx); Ok(())},
            TreeReturn::SetTarget(mesh) => {stage.set_camera_target(&mesh); Ok(())},
        };
    }

    if let Some(file_path) = new_obj_fp {
        stage.add_new_object(ctx, file_path)
    }
    stage.egui_mq.draw(ctx);
}
