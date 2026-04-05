#![allow(dead_code)]
//::///////////////////////////////////////////////////////////////////////////
//::
//::  Binary Generic File Format ID List
//::
//::  Copyright (c) 2006, BioWare Corp.
//::
//::///////////////////////////////////////////////////////////////////////////
//::
//::  BinaryGFFIDList.h
//::
//::///////////////////////////////////////////////////////////////////////////
//::
//::  Created By:       Ross Gardner
//::  Created On:       Feb 21, 2006
//::
//::  Maintained By:
//::
//::///////////////////////////////////////////////////////////////////////////
//::
//::  Description:      The Binary GFF system relies on IDs instead of string entries
//::                    for accessing and storing values with the GFF file.  This
//::                    requires that those IDs be assigned and stored in a central
//::                    place that is accessible by the system loading in those
//::                    files as well as the system that saves them out (creates
//::                    them in the first place)
//::
//::                    This file is a list of these IDs followed by a comment that
//::                    gives a name and a brief description about what the ID entry
//::                    is for.
//::
//::                    When creating a new GFF type reserve a range of IDs for that file
//::                    so that the IDs are easy to read. Reserving 1000 values should
//::                    be enough and will allow us to have over four million ranges
//::
//::                    Do not reuse IDs. If an ID needs to be removed then comment it out and
//::                    leave it there. Add new ranges at the end of all the ranges and new fields
//::                    in a range after all the other fields.
//::
//::  Format:           The following is the format of each entry:
//::
//::            GFF_****  =  ##, // $$ | $$$$
//::
//::                    where:
//::
//::            **** -is the remainder of the #define name that will be used in code
//::                  to read the field.
//::            ##   -is the number (ID) of the entry, should be exactly 1 more than
//::                  the previous entry (entries start at 1 and 0 is always invalid).
//::                  Do not leave this field blank.
//::            $$   -is the name of the entry as you want it to show up in the GFF
//::                  editor.  If nothing is entered it will use the **** entry as is.
//::            $$$$ -is what will show up in the tooltip on the entry in the GFF
//::                  editor.  It should be a very brief description of the field.
//::                  If nothing is entered then the tool tip will be blank.
//::
//::
//::///////////////////////////////////////////////////////////////////////////

/// Invalid Entry | Invalid Entry
pub const INVALIDENTRY: u32 = 0;

/// Tag | object tag - should be unique
pub const TAG: u32 = 1;

/// Name | the name of the object - as it shows up in
pub const NAME: u32 = 2;

/// Template Reference | The template file for this object
pub const TEMPLATERESREF: u32 = 3;

/// Position | The position of this object
pub const POSITION: u32 = 4;

/// Orientation | The orientation of this object
pub const ORIENTATION: u32 = 5;

/// UINT8 List | UINT8 List
pub const UINT8_LIST: u32 = 6;

/// INT8 List | INT8 List
pub const INT8_LIST: u32 = 7;

/// UINT16 List | UINT16 List
pub const UINT16_LIST: u32 = 8;

/// INT16 List | INT16 List
pub const INT16_LIST: u32 = 9;

/// UINT32 List | UINT32 List
pub const UINT32_LIST: u32 = 10;

/// INT32 List | INT32 List
pub const INT32_LIST: u32 = 11;

/// UINT64 List | UINT64 List
pub const UINT64_LIST: u32 = 12;

/// INT64 List | INT64 List
pub const INT64_LIST: u32 = 13;

/// FLOAT32 List | FLOAT32 List
pub const FLOAT32_LIST: u32 = 14;

/// FLOAT64 List | FLOAT64 List
pub const FLOAT64_LIST: u32 = 15;

/// Vector3f List | Vector3f List
pub const VECTOR3F_LIST: u32 = 16;

/// Vector4f List | Vector4f List
pub const VECTOR4F_LIST: u32 = 17;

/// Quaternionf List | Quaternionf List
pub const QUATERNIONF_LIST: u32 = 18;

/// ECString List | ECString List
pub const ECSTRING_LIST: u32 = 19;

/// Color4f List | Color4f List
pub const COLOR4F_LIST: u32 = 20;

/// Name hash | the hash value of the object's name
pub const NAME_HASH: u32 = 21;

/// Text | Localizable text
pub const TEXT: u32 = 22;

/// Object Id | The object id for this object.
pub const OBJECT_ID: u32 = 23;

/// Field for adding saved toolset property | Toolset property
pub const TS_PROPERTY: u32 = 900;

/// Property name for toolset property auto-save | Property name
pub const TS_PROPERTY_NAME: u32 = 901;

/// Property atom for toolset property auto-save | Property atom
pub const TS_PROPERTY_ATOM: u32 = 902;

/// Base Item ID | base Id of the item
pub const ITEM_BASEID: u32 = 1000;

/// Item Cost | cost of the item
pub const ITEM_COST: u32 = 1001;

/// Stack Size | maximum stack size of the item
pub const ITEM_STACKSIZE: u32 = 1002;

/// Item Stolen | true(1) or false(0) if the item can be stolen or not
pub const ITEM_STOLEN: u32 = 1003;

/// Plot Item | tru(e(1) or false(0) if it is a plot item or not
pub const ITEM_PLOT: u32 = 1004;

/// Number of Charges | number of charges this item starts out with
pub const ITEM_CHARGES: u32 = 1006;

/// Model Variation | variation number on the item model - combines in 3 digit format to complete the name  ie. 001
pub const ITEM_MODELVARIATION: u32 = 1007;

/// Property List | List of properties on the item
pub const ITEM_PROPERTYLIST: u32 = 1009;

/// Designer material | Index into materialtypes 2da
pub const ITEM_MATERIAL: u32 = 1010;

/// Item ability Id | Ability this item can use
pub const ITEM_ABILITYID: u32 = 1011;

/// List of sub-item resrefs | List of resrefs of items attached/contained in this item
pub const ITEM_SUBITEMS_RESREFS: u32 = 1019;

/// The base cost of an item
pub const ITEM_BASECOST: u32 = 1021;

/// Param1 | A parameter that can be used in certain properties
pub const ITEM_PROP_PARAM1: u32 = 2000;

/// Name | The name of the property
pub const ITEM_PROP_PROPERTYNAME: u32 = 2001;

/// Subtype | The subtype of the property
pub const ITEM_PROP_SUBTYPE: u32 = 2002;

/// Cost Table | Which cost table to use?
pub const ITEM_PROP_COSTTABLE: u32 = 2003;

/// Cost Value | The value of the cost?
pub const ITEM_PROP_COSTVALUE: u32 = 2004;

/// Param1 Value | The value of param1 if it is used
pub const ITEM_PROP_PARAM1VALUE: u32 = 2005;

/// Chance Appear | The chance for this item to appear
pub const ITEM_PROP_CHANCEAPPEAR: u32 = 2006;

/// World | struct "WRLD". top-level world layout struct
pub const ENV_WORLD: u32 = 3000;

/// name | string. Name of World as seen in env editor
pub const ENV_WORLD_NAME: u32 = 3001;

/// AreaList | list of area structs
pub const ENV_WORLD_AREA_LIST: u32 = 3002;

/// ChildList | Level Object Child List
pub const LVL_CHILD_LIST: u32 = 3003;

/// LVL File object version.
pub const LVL_FILE_OBJECT_VERSION: u32 = 3004;

/// LVL document last change time.
pub const LVL_CHANGETIME: u32 = 3005;

/// Area | struct "AREA" representing an Area Layout
pub const ENV_AREA: u32 = 3010;

/// id | int32 Area ID
pub const ENV_AREA_ID: u32 = 3011;

/// name | string. Name of Area Layout as seen in env editor
pub const ENV_AREA_NAME: u32 = 3012;

/// file | string.
pub const ENV_AREA_FILE: u32 = 3013;

/// RoomList | list of room structs
pub const ENV_AREA_ROOM_LIST: u32 = 3016;

/// position | vector. position of area
pub const ENV_AREA_POSITION: u32 = 3018;

/// rotation | quaternion. orientatio of area
pub const ENV_AREA_ROTATION: u32 = 3019;

/// PathfindingVisInfo | list of int32 areaIDs
pub const ENV_AREA_PATHFINDING_VISINFO: u32 = 3021;

/// Struct containing FB info (see GFF_CUTSCENE_ACTION_FRAME_BUFFER_EFFECT_ for contents)
pub const ENV_AREA_FRAME_BUFFER_EFFECT: u32 = 3023;

/// Area center | vector. center point of area
pub const ENV_AREA_CENTER: u32 = 3024;

/// Area skydome | resref for the skydome model
pub const ENV_AREA_SKYDOME_MODEL: u32 = 3025;

/// ResRef to global WND file
pub const ENV_AREA_GLOBALWIND_RESREF: u32 = 3027;

/// List of local WND files
pub const ENV_AREA_LOCALWIND_LIST: u32 = 3028;

/// list of ints indicating cost of pathfinding points.
pub const ENV_AREA_PATHFINDING_COSTS: u32 = 3029;

/// Room | struct "ROOM".
pub const ENV_ROOM: u32 = 3030;

/// id | int32. Room ID
pub const ENV_ROOM_ID: u32 = 3031;

/// name | string. Name of Room as seen in Env Editor
pub const ENV_ROOM_NAME: u32 = 3032;

/// file | string. extensionless filename of room layout file
pub const ENV_ROOM_FILE: u32 = 3033;

/// Connection | deprecated
pub const ENV_ROOM_PATHCONNECTION: u32 = 3045;

/// id | deprecated
pub const ENV_ROOM_PATHCONNECTION_ID: u32 = 3046;

/// Visible | deprecated
pub const ENV_ROOM_VISIBILITY: u32 = 3048;

/// id | deprecated
pub const ENV_ROOM_VISIBILITY_ID: u32 = 3049;

/// ModelList | list of roommodel structs.
pub const ENV_ROOM_MODEL_LIST: u32 = 3050;

/// LightList | list of roomlight structs
pub const ENV_ROOM_LIGHT_LIST: u32 = 3051;

/// DynShadowsEnabled | int8 treated as boolean, indicates if room has dynamic shadows
pub const ENV_ROOM_DYNSHADOW_ENABLED: u32 = 3055;

/// Overlapped | int8 boolean indicating if this model may overlap other models for pathfinding
pub const ENV_MODEL_PATHFINDING_OVERLAPPED: u32 = 3056;

/// Show High LOD | boolean indicating if this model should render in the editor with high LOD instead of low
pub const ENV_MODEL_SHOW_HIGH_LOD: u32 = 3057;

/// Snap To Terrain | boolean indicating if this model should snap to terrain as it is painted or moved
pub const ENV_MODEL_SNAP_TO_TERRAIN: u32 = 3058;

/// Model | struct "MDL".
pub const ENV_MODEL: u32 = 3060;

/// id | int32 model ID
pub const ENV_MODEL_ID: u32 = 3061;

/// name | string. Name of model object used by graphics engine.
pub const ENV_MODEL_NAME: u32 = 3062;

/// file | string. Name of model mmh file without extension.
pub const ENV_MODEL_FILE: u32 = 3063;

/// Light | struct "LIT"
pub const ENV_LIGHT: u32 = 3067;

/// id | int32 light ID
pub const ENV_LIGHT_ID: u32 = 3068;

/// name | string. Name of light object used by graphics engine
pub const ENV_LIGHT_NAME: u32 = 3069;

/// color | vector. color of the light
pub const LIGHT_COLOR: u32 = 3072;

/// light type | int32 enumerated value. See ILight.h for values
pub const LIGHT_TYPE: u32 = 3074;

/// point light radius | float32 radius of point lights.
pub const LIGHT_POINT_RADIUS: u32 = 3075;

/// baked | int8 boolean. True for baked light used for lightmapping, false for non-baked lights used in game
pub const LIGHT_BAKED: u32 = 3077;

/// effect | Light effect (replaces GFF_LIGHT_ISDYNAMIC and GFF_LIGHT_BAKED in new files)
pub const LIGHT_EFFECT: u32 = 3078;

/// character/level light | int32 enumerated value. See class LvlLight in RenderLvlObject.h
pub const LIGHT_AFFECT_DOMAIN: u32 = 3079;

/// NavInfo | struct "NAVI". top-level NavInfo struct for roomgrid pathfinding info
pub const AREAGRID_NAVINFO: u32 = 3080;

/// RoomName | string.
pub const AREAGRID_ROOMNAME: u32 = 3081;

/// GridNavInfo | list of ModelGrid structs
pub const AREAGRID_GRIDNAVINFO: u32 = 3082;

/// ModelGrid | struct "MDGD" containing model grid info
pub const AREAGRID_MODELGRID: u32 = 3083;

/// GridID | int32 ID of a modelgrid
pub const AREAGRID_GRIDID: u32 = 3084;

/// Columns | int32 number of columns
pub const AREAGRID_NBCOLUMNS: u32 = 3086;

/// Rows | int32 number of row
pub const AREAGRID_NBROWS: u32 = 3087;

/// CellSize | float32 size of each pathfinding cell in the grid
pub const AREAGRID_CELLSIZE: u32 = 3088;

/// HeightClearance | float32 height clearance of each cell
pub const AREAGRID_CLEARANCE: u32 = 3089;

/// BasePosition | vector position of the model grid origin corner (relative to the room?)
pub const AREAGRID_BASEPOS: u32 = 3090;

/// Normal | vector normal to the grid surface
pub const AREAGRID_NORMAL: u32 = 3091;

/// Data | float32 list of grid data. interpret rows and colums according to the NBROWS and NBCOLUMNS
pub const AREAGRID_DATA: u32 = 3092;

/// Data | float32 list of grid data. interpret rows and colums according to the NBROWS and NBCOLUMNS
pub const AREAGRID_HEIGHT: u32 = 3093;

/// Size | Abstraction Layer Sector Size
pub const AREAGRID_ABSTRACTION_SECTORSIZE: u32 = 3094;

/// Sector Data | Abstraction layer sector data
pub const AREAGRID_ABSTRACTION_SECTORS: u32 = 3095;

/// Memory Data | Abstraction layer memory data
pub const AREAGRID_ABSTRACTION_MEMORY: u32 = 3096;

/// Id | int32 link ID
pub const AREAGRID_ID: u32 = 3097;

/// CellId | int32 cell ID
pub const AREAGRID_CELLID: u32 = 3098;

/// spot distance | float32 distance of target from light
pub const LIGHT_SPOT_DISTANCE: u32 = 3102;

/// Light Probe | struct "PRB"
pub const ENV_LIGHT_PROBE: u32 = 3103;

/// Environment Map Resource Name | string
pub const ENV_LIGHT_PROBE_ENVMAP: u32 = 3104;

/// Soft Shadows | number of rays
pub const ENV_LIGHT_NUM_SAMPLES: u32 = 3105;

/// Soft Shadows | size of light emitter
pub const ENV_LIGHT_SIZE: u32 = 3106;

/// Dynamic shadow vector, as used by the game.  Different than the dynamic shadow data saved/loaded by the toolset
pub const DYNAMICSHADOW_VECTOR_GAME: u32 = 3107;

/// cutaway override | uint8 : 0 - no override; 1 - all parts do not cut away; 2 - all parts cut away
pub const ENV_MODEL_CUT_AWAY_OVERRIDE: u32 = 3109;

/// Struct "AREA" | top level struct for area pathfinding info
pub const AREAGRID_AREA: u32 = 3110;

/// Data | byte list of grid data, sound material type. interpret rows and colums according to the NBROWS and NBCOLUMNS
pub const AREAGRID_SOUND_DATA: u32 = 3114;

/// Light subset data | byte list of light subset (of containing room) that affects this point, interpret rows and colums according to the NBROWS and NBCOLUMNS
pub const AREAGRID_LIGHT_SUBSET_DATA8: u32 = 3117;

/// Light subset data | ushort list of light subset (of containing room) that affects this point, interpret rows and colums according to the NBROWS and NBCOLUMNS
pub const AREAGRID_LIGHT_SUBSET_DATA16: u32 = 3118;

/// Can be occluded | bool indicating whether light can be occluded (character lights only as of 2008.10.22)
pub const LIGHT_CAN_BE_OCCLUDED: u32 = 3119;

/// CellPading | float32 padding distance at the lower corner of the pathfinding patch
pub const AREAGRID_CELLPADDING: u32 = 3120;

/// ischunk | int8, 1 for chunks, 0 for normal room. If chunked, the roomlist must have rowcount*colcount elements.
pub const ENV_AREA_CHUNK_ISCHUNK: u32 = 3122;

/// chunk row count | int32 number of rows (y) of chunks
pub const ENV_AREA_CHUNK_ROWCOUNT: u32 = 3123;

/// chunk width | float32 width (x-axis) of each chunk
pub const ENV_AREA_CHUNK_WIDTH: u32 = 3125;

/// chunk height | float32 height (y axis) of each chunk
pub const ENV_AREA_CHUNK_HEIGHT: u32 = 3126;

/// start point name of the area
pub const ENV_AREA_STARTPOINT_NAME: u32 = 3128;

/// cutoff height | enumerated high/med/low universal cutoff height for the level.
pub const ENV_AREA_CUTOFF_HEIGHT: u32 = 3129;

/// animated maximum frequency | maximum frequency of intensity change for an animated light
pub const LIGHT_ANIMATED_MIN_FREQUENCY: u32 = 3130;

/// animated maximum frequency | maximum frequency of intensity change for an animated light
pub const LIGHT_ANIMATED_MAX_FREQUENCY: u32 = 3131;

/// animated maximum frequency | maximum frequency of intensity change for an animated light
pub const LIGHT_ANIMATED_MIN_INTENSITY: u32 = 3132;

/// animated maximum frequency | maximum frequency of intensity change for an animated light
pub const LIGHT_ANIMATED_MAX_INTENSITY: u32 = 3133;

/// whether the cutoff system is enabled (DA1 defaults: exterior=false, interior=true)
pub const ENV_AREA_CUTOFF_SYSTEM_ENABLED: u32 = 3134;

/// World coordinates of lower left point in minimap.
pub const ENV_MINIMAP_LOWER_LEFT_POINT: u32 = 3138;

/// ---DEPRECATED--- Temporary flag for forcing character lighting. In prep. for E3 2008.
pub const ENV_AREA_FORCE_CHARACTER_LIGHTING: u32 = 3142;

/// Character sunlight can be occluded
pub const ENV_AREA_SUNLIGHT_CAN_BE_OCCLUDED_CHAR: u32 = 3148;

/// Character sunlight color
pub const ENV_AREA_SUNLIGHT_COLOR_CHAR: u32 = 3149;

/// SunlightColor | Color, the color of the sunlight
pub const ENV_AREA_SUNLIGHT_COLOR: u32 = 3152;

/// Terrain Chunk | Struct defining a chunk of a terrain level
pub const TERRAIN_CHUNK: u32 = 3154;

/// Terrain Chunk List | List of structs of chunk objects.
pub const TERRAIN_CHUNK_LIST: u32 = 3155;

/// Chunk Length | Length of the chunk
pub const TERRAIN_CHUNK_LENGTH: u32 = 3158;

/// Chunk Width  | Width of the chunk
pub const TERRAIN_CHUNK_WIDTH: u32 = 3159;

/// Chunk Texel Size | Texel size of the chunk
pub const TERRAIN_CHUNK_TEXEL_SIZE: u32 = 3160;

/// Sector ID | Sector ID related to the chunk
pub const TERRAIN_CHUNK_SECTOR_ID: u32 = 3162;

/// Distance Fog Color | Vector3f - Color of the distance fog
pub const ENV_FOG_COLOR: u32 = 3165;

/// Distance Fog Max Distance | FLOAT32 - The distance at which the fog reach max density (from the camera)
pub const ENV_FOG_MAX_DISTANCE: u32 = 3166;

/// Distance Fog Min Distance | FLOAT32 - The distance from the camera at which the the fog begins
pub const ENV_FOG_MIN_DISTANCE: u32 = 3169;

/// name changed | boolean indicating if this model name was changed by the user.
pub const ENV_MODEL_NAME_CHANGED: u32 = 3170;

/// vegetation | struct "VEGT"
pub const ENV_VEGETATION: u32 = 3171;

/// test creature | struct "CRE"
pub const ENV_CREATURE: u32 = 3172;

/// Camera | struct Camera info
pub const ENV_CAMERA: u32 = 3200;

/// Camera Pivot Distance | float32 camera pivot distance
pub const ENV_CAMERA_PIVOTDISTANCE: u32 = 3201;

/// Standalone Children | struct of children
pub const ENV_STANDALONE: u32 = 3202;

/// Standalone Area List | list of Area structs
pub const ENV_LIST_AREA: u32 = 3202;

/// Standalone Room List | list of Room structs
pub const ENV_LIST_ROOM: u32 = 3203;

/// Standalone Model List | list of Model structs
pub const ENV_LIST_MODEL: u32 = 3204;

/// Standalone Light List | list of Light structs
pub const ENV_LIST_LIGHT: u32 = 3205;

/// Pathfinding container layout name.
pub const ENV_PFCONTAINER_LAYOUTNAME: u32 = 3210;

/// Pathfinding container export data.
pub const ENV_PFCONTAINER_EXPORTDATA: u32 = 3211;

/// Pathfinding container export data version.
pub const ENV_PFCONTAINER_DATAVERSION: u32 = 3212;

/// Pathfinding container visualization info.
pub const ENV_PFCONTAINER_VISINFO: u32 = 3213;

/// RIMTree Root Node | struct, Root of Resource RIM Tree
pub const RIMTREE_ROOT_NODE: u32 = 3290;

/// RIM List | list of RIM names, RIMs required for node
pub const RIMTREE_RIM_LIST: u32 = 3291;

/// Child List | list of RIMTree Node structs, children of node
pub const RIMTREE_CHILD_LIST: u32 = 3292;

/// Node TAG | string, TAG of node
pub const RIMTREE_NODE_TAG: u32 = 3293;

/// Node ResRef | string, ResRef of node
pub const RIMTREE_NODE_RESREF: u32 = 3294;

/// Group | struct, a group of level objects
pub const ENV_GROUP: u32 = 3300;

/// Group Name | string, the name of a group
pub const ENV_GROUP_NAME: u32 = 3301;

/// Start Point Group | struct, a group of Start Point
pub const ENV_SP_GROUP: u32 = 3302;

/// Start Point Group Name | string, the name of group
pub const ENV_SP_GROUP_NAME: u32 = 3303;

/// Start Point | struct
pub const ENV_SP: u32 = 3304;

/// Start Point Name | string
pub const ENV_SP_FILE: u32 = 3305;

/// LockSelection | int32 enumeration. 0 = normal, 1 = unselectable, 2 = exclusive select
pub const ENV_OBJECT_LOCKSELECTION: u32 = 3311;

/// InstanceID | string to pass to Graphics::IBaseObject::SetInstanceID() after creating an object
pub const ENV_MODEL_INSTANCEID: u32 = 3320;

/// LightmapOffsetAndScale | vector4 containing lightmap data offset and scale to use when accessing atlas texture
pub const ENV_MODEL_LIGHTMAP_OFFSET_SCALE: u32 = 3324;

/// Filename for the lightmap
pub const LVL_LIGHTMAP_FILESPEC: u32 = 3333;

/// Lighting Version for point light selection algorithm.
pub const LVL_LIGHTING_VERSION: u32 = 3334;

/// Ambient Occlusion Min Color | color3 Ambient Occlusion
pub const LVL_AO_COLOR_MIN: u32 = 3340;

/// Ambient Occlusion Max Color | color3 Ambient Occlusion
pub const LVL_AO_COLOR_MAX: u32 = 3341;

/// Ambient Occlusion Min Samples | int32 Ambient Occlusion
pub const LVL_AO_SAMPLES_MIN: u32 = 3342;

/// Ambient Occlusion Max Samples | int32 Ambient Occlusion
pub const LVL_AO_SAMPLES_MAX: u32 = 3343;

/// Ambient Occlusion Cone Angle | float32 Ambient Occlusion
pub const LVL_AO_CONEANGLE: u32 = 3347;

/// Ambient Occlusion Exponent | float32 Ambient Occlusion
pub const LVL_AO_EXPONENT: u32 = 3349;

/// Model | struct "MDL".
pub const ENV_TREE: u32 = 3350;

/// id | int32 model ID
pub const ENV_TREENODE_ID: u32 = 3351;

/// name | string. Name of model object used by graphics engine.
pub const ENV_TREE_NAME: u32 = 3352;

/// file | string. Name of model mmh file without extension.
pub const ENV_TREE_FILE: u32 = 3353;

/// TreeNodeList | list of room tree node structs.
pub const ENV_ROOM_TREENODE_LIST: u32 = 3354;

/// Global resource ID, used by SPT instance
pub const ENV_AREA_TREECONTROLLER_ID: u32 = 3357;

/// Painted Tree List | struct "TLST"
pub const ENV_TREE_PAINTED_LIST: u32 = 3358;

/// Painted tree position | vector3
pub const ENV_TREE_PAINTED_POSITION: u32 = 3359;

/// Painted tree rotation around z axis | float32
pub const ENV_TREE_PAINTED_ROTATION: u32 = 3360;

/// Painted tree scaling factor | float32
pub const ENV_TREE_PAINTED_SCALE: u32 = 3361;

/// Scatter object | struct "SCAT"
pub const ENV_SCATTER_OBJECTS: u32 = 3362;

/// file | string | Scatter objects .mmh prototype file.
pub const ENV_SCATTEROBJECT_FILE: u32 = 3363;

/// Scatter object instance
pub const ENV_SCATTER_INSTANCE: u32 = 3364;

/// Scatter object instance list
pub const ENV_SCATTER_INSTANCE_LIST: u32 = 3365;

/// ScatterObjectList | List of room scatter objects
pub const ENV_SCATTEROBJECT_LIST: u32 = 3366;

/// string | Scatter objects prototype model (.mmh)
pub const ENV_SCATTEROBJ_PROTOTYPE: u32 = 3373;

/// string | Scatter objects instance data file (.msi)
pub const ENV_SCATTEROBJ_MSI_DATA: u32 = 3374;

/// color | tinting color for a tree
pub const ENV_TREE_COLOR_TINT: u32 = 3375;

/// color | tinting color for trees in an area. On export this represents a Vector4 with color XYZ and the intensity in W
pub const ENV_TREE_COLOR_LEVEL_TINT: u32 = 3377;

/// float | multiplies the color values in the tint. This ID isn't used for export, the value is exported in the ID above.
pub const ENV_TREE_COLOR_LEVEL_INTENSITY: u32 = 3378;

/// Terrain Export Area | Struct defining an exportable region of a terrain level
pub const TERRAIN_EXPORT_AREA: u32 = 3400;

/// Cell X Size | Number of Cells in X direction
pub const TERRAIN_AREA_CELL_SIZE_X: u32 = 3406;

/// Cell Y Size | Number of Cells in Y direction
pub const TERRAIN_AREA_CELL_SIZE_Y: u32 = 3407;

/// Cell Z Size | Number of Cells in Z direction
pub const TERRAIN_AREA_CELL_SIZE_Z: u32 = 3408;

/// Terrain Export Area Vista Lightmap Size | Width of lightmap textures for vista chunks
pub const TERRAIN_AREA_LIGHTMAP_SIZE_VISTA: u32 = 3412;

/// Terrain Export Area Chunk Subdivision Factor | Number of times to subdivide each chunk on export.
pub const TERRAIN_AREA_SUBDIVIDE_BY: u32 = 3413;

/// Part group this model belongs to.
pub const ENV_MODEL_PARTGROUP: u32 = 3500;

/// Lightmap only flag for a model.
pub const ENV_MODEL_LIGHTMAPONLY: u32 = 3501;

/// Lightmap enable
pub const ENV_MODEL_LIGHTMAP_FLAG: u32 = 3502;

/// Export enable
pub const ENV_MODEL_EXPORT_FLAG: u32 = 3503;

/// The name of the looping animation to play.
pub const ENV_MODEL_DEFAULT_ANIMATION: u32 = 3504;

/// The name of the blend tree the default animation is in (should be based on the chunk/room)
pub const ENV_MODEL_BLEND_TREE_NAME: u32 = 3505;

/// User param list | list of user param structs.
pub const ENV_MODEL_USER_PARAM_LIST: u32 = 3506;

/// User param name | the name of the user param
pub const ENV_MODEL_USER_PARAM_NAME: u32 = 3507;

/// User param value | the value of the user param
pub const ENV_MODEL_USER_PARAM_VALUE: u32 = 3508;

/// Water | struct "AQUA".
pub const LVL_WATER: u32 = 3600;

/// float | Water quad dimension in the X axis
pub const LVL_WATER_SIZE_X: u32 = 3601;

/// float | Water quad dimension in the Y axis
pub const LVL_WATER_SIZE_Y: u32 = 3602;

/// int | Water max tessellation level
pub const LVL_WATER_MAX_TESSELLATION: u32 = 3603;

/// int | Water mesh ID
pub const LVL_WATER_MESH_ID: u32 = 3604;

/// string | Water waves normal map
pub const LVL_WATER_NORMAL_MAP: u32 = 3605;

pub const LVL_WATER_HEIGHT_MAP: u32 = 3606;

/// Color | Deep Water Color
pub const LVL_WATER_DEEP_COLOR: u32 = 3607;

/// Color | Shallow Water Color
pub const LVL_WATER_SHALLOW_COLOR: u32 = 3608;

/// float | Wave 0 frequency
pub const LVL_WATER_WAVE_FREQ_1: u32 = 3609;

/// float | Wave 0 amplitude
pub const LVL_WATER_WAVE_AMPL_1: u32 = 3610;

/// float | Wave 0 direction angle
pub const LVL_WATER_WAVE_DIRECTION_1: u32 = 3611;

/// float | Wave 0 direction angle
pub const LVL_WATER_WAVE_SPEED_1: u32 = 3618;

/// float | Wave 1 frequency
pub const LVL_WATER_WAVE_FREQ_2: u32 = 3612;

/// float | Wave 1 amplitude
pub const LVL_WATER_WAVE_AMPL_2: u32 = 3613;

/// float | Wave 1 direction angle
pub const LVL_WATER_WAVE_DIRECTION_2: u32 = 3614;

/// float | Wave 0 direction angle
pub const LVL_WATER_WAVE_SPEED_2: u32 = 3619;

/// float | Wave 2 frequency
pub const LVL_WATER_WAVE_FREQ_3: u32 = 3615;

/// float | Wave 2 amplitude
pub const LVL_WATER_WAVE_AMPL_3: u32 = 3616;

/// float | Wave 2 direction angle
pub const LVL_WATER_WAVE_DIRECTION_3: u32 = 3617;

/// float | Wave 0 direction angle
pub const LVL_WATER_WAVE_SPEED_3: u32 = 3620;

pub const LVL_WATER_REFLECTIVITY: u32 = 3621;

pub const LVL_WATER_FOAM_HEIGHT: u32 = 3622;

pub const LVL_WATER_SUBDIVISION_DEPTH_TOLERANCE: u32 = 3623;

/// float | shallow water depth
pub const LVL_WATER_SHALLOW_DEPTH: u32 = 3624;

pub const LVL_WATER_FOAM_COLOR: u32 = 3625;

/// float | The maximum depth walkable for characters
pub const LVL_WATER_WALKABLE_DEPTH: u32 = 3626;

pub const LVL_WATER_OPACITY_FALLOFF: u32 = 3628;

pub const LVL_WATER_SUNLIGHT_SPECULAR_POWER: u32 = 3629;

pub const LVL_WATER_SPECULAR_MULTIPLIER: u32 = 3630;

pub const LVL_WATER_SPECULAR_FALLOFF: u32 = 3631;

pub const LVL_WATER_COLORIZE_TRANSPARENCY: u32 = 3632;

pub const LVL_WATER_OVERRIDE_REFLECTION: u32 = 3633;

pub const LVL_WATER_ENABLE_SPEC: u32 = 3634;

/// struct
pub const LVL_WIND: u32 = 3700;

/// int32 | ID for the wind object.
pub const LVL_WIND_ID: u32 = 3701;

/// string | User defined object name.
pub const LVL_WIND_NAME: u32 = 3702;

/// bool | This wind object is global
pub const LVL_WIND_ISGLOBAL: u32 = 3710;

/// float |
pub const LVL_WIND_REGIONRADIUS: u32 = 3711;

/// float |
pub const LVL_WIND_REGIONFALLOFF: u32 = 3712;

/// float |
pub const LVL_WIND_SPTSTRENGTH: u32 = 3713;

/// float |
pub const LVL_WIND_SPTGUST_MINPERCENT: u32 = 3714;

/// float |
pub const LVL_WIND_SPTGUST_MAXPERCENT: u32 = 3715;

/// float |
pub const LVL_WIND_SPTGUST_MINDURATION: u32 = 3716;

/// float |
pub const LVL_WIND_SPTGUST_MAXDURATION: u32 = 3717;

/// float |
pub const LVL_WIND_SPTBENDANGLE: u32 = 3718;

/// float |
pub const LVL_WIND_CLOTH_RESPONSE: u32 = 3719;

/// float |
pub const LVL_WIND_CLOTH_RESPONSE_LMT: u32 = 3720;

/// float |
pub const LVL_WIND_CLOTH_STRENGTH: u32 = 3721;

/// float |
pub const LVL_WIND_CLOTH_GUST_STRENGTH_MIN: u32 = 3722;

/// float |
pub const LVL_WIND_CLOTH_GUST_STRENGTH_MAX: u32 = 3723;

/// float |
pub const LVL_WIND_CLOTH_GUST_DURATION_MIN: u32 = 3724;

/// float |
pub const LVL_WIND_CLOTH_GUST_DURATION_MAX: u32 = 3725;

/// float |
pub const LVL_WIND_CLOTH_GUST_INTERVAL_MIN: u32 = 3726;

/// float |
pub const LVL_WIND_CLOTH_GUST_INTERVAL_MAX: u32 = 3727;

/// float |
pub const LVL_WIND_CLOTH_GUST_DIR_CHANGE: u32 = 3728;

/// vector3 |
pub const LVL_WIND_CLOTH_GUST_AXIS_RATIO: u32 = 3729;

pub const LVL_WIND_SPTGUST_FREQUENCY: u32 = 3730;

/// Collision wall information
pub const LVL_COLLISION_WALL_INFO: u32 = 3730;

/// uint32 List
pub const LVL_COLLISION_WALL_VERTICIES: u32 = 3731;

/// Vector3f List
pub const LVL_COLLISION_WALL_VERTICIES_V2: u32 = 3732;

/// Lower x edge of the minimap region | float
pub const LVL_MINIMAP_POSITION_X: u32 = 3740;

/// Lower y edge of the minimap region | float
pub const LVL_MINIMAP_POSITION_Y: u32 = 3741;

/// x size of the minimap region | float
pub const LVL_MINIMAP_SIZE_X: u32 = 3742;

/// y size of the minimap region | float
pub const LVL_MINIMAP_SIZE_Y: u32 = 3743;

/// Optimized static physics blob
pub const ENV_STAT_PHYS: u32 = 3744;

/// Optimized static physics data
pub const ENV_STAT_PHYS_DATA: u32 = 3745;

/// Light Subset | a list of light IDs (the subset of all lights that affect a point)
pub const LVL_LIGHT_SUBSET_LIST: u32 = 3800;

/// Node Name | The name of the animation node  // DEPRECATED
pub const ANIMATION_NODENAME: u32 = 4000;

/// Target | The target type of the animation data
pub const ANIMATION_TARGET: u32 = 4001;

/// Source Type | The type of the animation source
pub const ANIMATION_SOURCETYPE: u32 = 4002;

/// Elements | The number of elements per entry
pub const ANIMATION_ELEMENTSPERENTRY: u32 = 4003;

/// Node Data | The data for this animation node
pub const ANIMATION_NODEDATA: u32 = 4004;

/// Node List | The list of node for this animation
pub const ANIMATION_NODELIST: u32 = 4005;

/// Name | The file name of the animation  // DEPRECATED
pub const ANIMATION_NAME: u32 = 4006;

/// Length | The length in time of the animation
pub const ANIMATION_ANIMLENGTH: u32 = 4009;

/// Is Additive | Is this animation additive
pub const ANIMATION_ISADDITIVE: u32 = 4011;

/// Is Override | Is this an override animation
pub const ANIMATION_ISOVERRIDE: u32 = 4012;

/// Animation event time | The time of the animation event
pub const ANIMATION_EVENT_TIME: u32 = 4016;

/// Animation event list | List of animation events
pub const ANIMATION_EVENT_LIST: u32 = 4020;

/// Animation tree node list | List of animation tree nodes
pub const ANIMATION_TREE: u32 = 4021;

/// Animation tree name | Name of the animation blend tree
pub const ANIMATION_TREE_NAME: u32 = 4022;

/// Animation tree node list | List of animation tree nodes
pub const ANIMATION_TREE_NODE: u32 = 4023;

/// Node name | Animation tree node name
pub const ANIMATION_TREE_NODE_NAME: u32 = 4024;

/// Animation filename | Animation tree node filename
pub const ANIMATION_TREE_NODE_FILE: u32 = 4025;

/// Node flags | Animation tree node flags
pub const ANIMATION_TREE_NODE_FLAGS: u32 = 4027;

/// Number of children | Number of children
pub const ANIMATION_TREE_NODE_NUM_CHILDREN: u32 = 4029;

/// Parent node | Index of parent node
pub const ANIMATION_TREE_NODE_PARENT: u32 = 4030;

pub const ANIMATION_BLENDCURVE_ANIMFROM: u32 = 4031;

pub const ANIMATION_BLENDCURVE_ANIMTO: u32 = 4032;

pub const ANIMATION_BLENDCURVE_DATA: u32 = 4033;

pub const ANIMATION_BLENDCURVE_LIST: u32 = 4034;

/// Keyframe time | Time of a keyframe
pub const ANIMATION_KEY_TIME: u32 = 4035;

/// Keyframe data | Data for a keyframe
pub const ANIMATION_KEY_DATA0: u32 = 4036;

/// Keyframe data | Data for a keyframe
pub const ANIMATION_KEY_DATA1: u32 = 4037;

/// Keyframe data | Data for a keyframe
pub const ANIMATION_KEY_DATA2: u32 = 4038;

/// Keyframe data | Data for a keyframe
pub const ANIMATION_KEY_DATA3: u32 = 4039;

/// Animation ignores animation scaling (true if cinematic animator animates for specific rigs)
pub const ANIMATION_IGNORESCALE: u32 = 4040;

/// RunTime | Running time of the cutscene
pub const CUTSCENE_RUN_TIME: u32 = 5000;

/// EndScript | Script to execute when the cutscene completes
pub const CUTSCENE_END_SCRIPT: u32 = 5001;

/// Layout | The layout the cutscene takes place in
pub const CUTSCENE_LAYOUT: u32 = 5002;

/// Position | The initial location of the camera
pub const CUTSCENE_POSITION: u32 = 5003;

/// Orientation | The initial orientation of the camera
pub const CUTSCENE_ORIENTATION: u32 = 5004;

/// Field of view | The initial field of view of the camera
pub const CUTSCENE_FOV: u32 = 5006;

/// Blend Tree | The custom blend tree for the cutscene
pub const CUTSCENE_BLENDTREE: u32 = 5007;

/// Animatic | The animatic to play instead of this cutscene
pub const CUTSCENE_ANIMATIC: u32 = 5008;

/// Staged | Whether the cutscene is staged
pub const CUTSCENE_STAGED: u32 = 5010;

/// LOD Curves | Position curves for the LOD object
pub const CUTSCENE_LOD_CURVES: u32 = 5011;

/// Anim Sound Events | Will animation sound events be played for actors in the scene.
pub const CUTSCENE_ANIM_SOUND_EVENTS: u32 = 5012;

/// FPS | Frames per second
pub const CUTSCENE_FPS: u32 = 5016;

/// Stage ResRef | Stage ResRef
pub const CUTSCENE_STAGE_RESREF: u32 = 5017;

/// Play Until VO Completes | Cutscene plays until VO completes, even if that is past the end
pub const CUTSCENE_PLAY_UNTIL_VO_COMPLETES: u32 = 5018;

/// Requires Area | This cutscene requires its area to be loaded in order to play properly
pub const CUTSCENE_AREA_REQUIRED: u32 = 5019;

/// Shadow Radius | Shadow Radius
pub const CUTSCENE_SHADOW_RADIUS: u32 = 5020;

/// Light Occlusion | Character Light Occlusion
pub const CUTSCENE_LIGHT_OCCLUSION: u32 = 5021;

/// Resources | List of cutscene resources
pub const CUTSCENE_RESOURCES: u32 = 5100;

/// ResRef | ResRef
pub const CUTSCENE_RESOURCE_RESREF: u32 = 5101;

/// Type | Resource Type
pub const CUTSCENE_RESOURCE_TYPE: u32 = 5102;

/// Actors | List of cutscene actors
pub const CUTSCENE_ACTORS: u32 = 5200;

/// ID | ID of the actor
pub const CUTSCENE_ACTOR_ID: u32 = 5201;

/// ModelRef | ResRef of the model
pub const CUTSCENE_ACTOR_MODEL_RESREF: u32 = 5202;

/// Deprecated | Deprecated
pub const CUTSCENE_ACTOR_DEPRECATED_1: u32 = 5203;

/// Deprecated | Deprecated
pub const CUTSCENE_ACTOR_DEPRECATED_2: u32 = 5204;

/// Deprecated | Deprecated
pub const CUTSCENE_ACTOR_DEPRECATED_3: u32 = 5205;

/// ActionQueue | List of actions for this actor
pub const CUTSCENE_ACTOR_ACTION_QUEUE: u32 = 5206;

/// Deprecated | Deprecated
pub const CUTSCENE_ACTOR_DEPRECATED_4: u32 = 5207;

/// Creature | ResRef of the creature
pub const CUTSCENE_ACTOR_CREATURE_RESREF: u32 = 5208;

/// Camera Target | Camera target actor ID
pub const CUTSCENE_ACTOR_CAMERA_TARGET: u32 = 5209;

/// Use Pose | Whether this actor uses poses or not
pub const CUTSCENE_ACTOR_USE_POSE: u32 = 5210;

/// Pose | The pose ID for this creature
pub const CUTSCENE_ACTOR_POSE: u32 = 5211;

/// Pose Speed | The pose speed for this creature
pub const CUTSCENE_ACTOR_POSE_SPEED: u32 = 5212;

/// Origin Orientation | The orientation to offset all movement and orientation from
pub const CUTSCENE_ACTOR_ORIGIN_ORI: u32 = 5215;

/// Inventory | The inventory bits
pub const CUTSCENE_ACTOR_INVENTORY: u32 = 5217;

/// Previous Pose | The previous pose to this line
pub const CUTSCENE_ACTOR_PREVIOUS_POSE: u32 = 5219;

/// Final Position | The final position to offset all movement and orientation from
pub const CUTSCENE_ACTOR_FINAL_POS: u32 = 5221;

/// Final Orientation | The final orientation to offset all movement and orientation from
pub const CUTSCENE_ACTOR_FINAL_ORI: u32 = 5222;

/// Master | Whether this is the master actor or not
pub const CUTSCENE_ACTOR_MASTER: u32 = 5223;

/// LOD | The LOD for this creature
pub const CUTSCENE_ACTOR_LOD: u32 = 5224;

/// Actor Model Scale | The Scale for Actor Model
pub const CUTSCENE_ACTOR_MODEL_SCALE: u32 = 5226;

/// Type | The type of action
pub const CUTSCENE_ACTION_TYPE: u32 = 5300;

/// StartTime | The starting time of the action
pub const CUTSCENE_ACTION_START_TIME: u32 = 5301;

/// StopTime | The stopping time of the action
pub const CUTSCENE_ACTION_STOP_TIME: u32 = 5302;

/// Curves | Curves on the action
pub const CUTSCENE_ACTION_CURVES: u32 = 5303;

/// Category | Action category
pub const CUTSCENE_ACTION_CATEGORY: u32 = 5304;

/// Base Value | Base value of the curve
pub const CUTSCENE_ACTION_CURVE_BASE_VALUE: u32 = 5350;

/// Vertices | List of vertices making up the curve
pub const CUTSCENE_ACTION_CURVE_VERTICES: u32 = 5351;

/// Deprecated | Deprecated
pub const CUTSCENE_ACTION_CURVE_DEPRECATED: u32 = 5353;

/// Time | Time of the vertex
pub const CUTSCENE_ACTION_CURVE_VERTEX_TIME: u32 = 5370;

/// Value | Value of the vertex
pub const CUTSCENE_ACTION_CURVE_VERTEX_VALUE: u32 = 5371;

/// Transition Type | Type of transition
pub const CUTSCENE_ACTION_CURVE_TRANSITION_TYPE: u32 = 5380;

/// Animation Name | Animation name
pub const CUTSCENE_ACTION_ANIM_ANIMATION_NAME: u32 = 5400;

/// Blend Tree Name | Blend tree name
pub const CUTSCENE_ACTION_ANIM_BLENDTREE_NAME: u32 = 5401;

/// Animation Speed | Animation speed
pub const CUTSCENE_ACTION_ANIM_SPEED: u32 = 5402;

/// Animation Start Offset | Starting time of the animation (offset from action start time)
pub const CUTSCENE_ACTION_ANIM_START_OFFSET: u32 = 5403;

/// Deprecated | Do not re-use this ID
pub const CUTSCENE_ACTION_ANIM_DEPRECATED1: u32 = 5404;

/// Link to Movement | Link animation speed to the velocity of the actor at this speed ratio.
pub const CUTSCENE_ACTION_ANIM_LINK_TO_MOVEMENT: u32 = 5407;

/// Blend GAD | Blend the GAD animation with out blending GADs (rather than adding)
pub const CUTSCENE_ACTION_ANIM_BLEND_GAD: u32 = 5410;

/// DEPRECATED
pub const CUTSCENE_ACTION_FRAME_BUFFER_EFFECT_FILE_NAME: u32 = 5520;

/// FBE Effect | Name of the FBE
pub const CUTSCENE_ACTION_FRAME_BUFFER_EFFECT_EFFECT_NAME: u32 = 5521;

/// VO Bank | Sound bank containing VO data
pub const CUTSCENE_ACTION_SPEAK_LINE_VOBANK: u32 = 5563;

/// Default Camera | Whether this is a camera tag or a default camera of a place tag
pub const CUTSCENE_ACTION_STAGE_CAMERA_DEFAULT_CAMERA: u32 = 5570;

/// Look At | The place to look at
pub const CUTSCENE_ACTION_STAGE_PLACE_LOOK_AT: u32 = 5580;

/// Shake Type | The type of shake
pub const CUTSCENE_ACTION_SHAKE_TYPE: u32 = 5600;

/// Deprecated | Do not re-use this ID
pub const CUTSCENE_ACTION_SHAKE_DEPRECATED1: u32 = 5601;

/// Deprecated | Do not re-use this ID
pub const CUTSCENE_ACTION_SHAKE_DEPRECATED2: u32 = 5602;

/// Seed | The seed for the noise
pub const CUTSCENE_ACTION_SHAKE_NOISE_SEED: u32 = 5603;

/// Frequency | The frequency for the noise
pub const CUTSCENE_ACTION_SHAKE_NOISE_FREQUENCY: u32 = 5604;

/// Type | The type of noise
pub const CUTSCENE_ACTION_SHAKE_NOISE_TYPE: u32 = 5605;

/// Roughness | The roughness of the noise
pub const CUTSCENE_ACTION_SHAKE_NOISE_ROUGHNESS: u32 = 5607;

/// Ramp-In | The ramp in time for the noise
pub const CUTSCENE_ACTION_SHAKE_NOISE_RAMP_IN: u32 = 5608;

/// Camera ID | Actor ID of the camera
pub const CUTSCENE_ACTION_ACTIVE_CAMERA_ACTOR_ID: u32 = 5610;

/// Target Id | Target Id
pub const CUTSCENE_ACTION_HEADTRACKING_TARGET_ID: u32 = 5620;

/// Headtracking speed | Headtracking speed
pub const CUTSCENE_ACTION_HEADTRACKING_SPEED: u32 = 5621;

/// Deprecated | Do not re-use this ID
pub const CUTSCENE_ACTION_HEADTRACKING_DEPRECATED1: u32 = 5624;

/// Deprecated | Do not re-use this ID
pub const CUTSCENE_ACTION_HEADTRACKING_DEPRECATED2: u32 = 5625;

/// Deprecated | Do not re-use this ID
pub const CUTSCENE_ACTION_HEADTRACKING_DEPRECATED3: u32 = 5626;

/// Deprecated | Do not re-use this ID
pub const CUTSCENE_ACTION_HEADTRACKING_DEPRECATED4: u32 = 5627;

/// Realign Source Head At Start | Whether we should force realign the source head at the start
pub const CUTSCENE_ACTION_HEADTRACKING_REALIGN_START: u32 = 5628;

/// Realign Source Head Continuously | Whether we should realign the source head continously
pub const CUTSCENE_ACTION_HEADTRACKING_REALIGN_CONT: u32 = 5629;

/// Node ID | The ID of the node to attach to
pub const CUTSCENE_ACTION_LINK_ACTOR_NODE_ID: u32 = 5631;

/// Deprecated | Do not re-use this ID
pub const CUTSCENE_ACTION_LINK_ACTOR_DEPRECATED1: u32 = 5632;

/// Deprecated | Do not re-use this ID
pub const CUTSCENE_ACTION_LINK_ACTOR_DEPRECATED2: u32 = 5633;

/// Deprecated | Do not re-use this ID
pub const CUTSCENE_ACTION_LINK_ACTOR_DEPRECATED3: u32 = 5634;

/// Deprecated | Do not re-use this ID
pub const CUTSCENE_ACTION_LINK_ACTOR_DEPRECATED4: u32 = 5635;

/// Is crust | If true, the target node ID refers to a crust and requires a lookup
pub const CUTSCENE_ACTION_LINK_ACTOR_IS_TARGET_CRUST: u32 = 5636;

/// Use Offset | Whether to use an offset
pub const CUTSCENE_ACTION_LINK_ACTOR_USE_OFFSET: u32 = 5637;

/// Pose | The pose blend tree
pub const CUTSCENE_ACTION_POSE_ANIMATION_POSE: u32 = 5650;

/// Animation | The pose animation
pub const CUTSCENE_ACTION_POSE_ANIMATION_ANIMATION: u32 = 5651;

/// Sound name | Sound event name
pub const CUTSCENE_ACTION_SOUND_NAME: u32 = 5670;

/// Sound param 1 | Sound param no. 1
pub const CUTSCENE_ACTION_SOUND_SPLINE_PARAM_NO1: u32 = 5671;

/// Sound param 2 | Sound param no. 2
pub const CUTSCENE_ACTION_SOUND_SPLINE_PARAM_NO2: u32 = 5672;

/// Sound param 3 | Sound param no. 3
pub const CUTSCENE_ACTION_SOUND_SPLINE_PARAM_NO3: u32 = 5673;

/// Sound param 4 | Sound param no. 4
pub const CUTSCENE_ACTION_SOUND_SPLINE_PARAM_NO4: u32 = 5674;

/// Sound param 5 | Sound param no. 5
pub const CUTSCENE_ACTION_SOUND_SPLINE_PARAM_NO5: u32 = 5675;

/// Cloth | Toggle cloth physics
pub const CUTSCENE_ACTION_TOGGLE_CLOTH_PHYSICS: u32 = 5700;

/// Hair | Toggle hair physics
pub const CUTSCENE_ACTION_TOGGLE_HAIR_PHYSICS: u32 = 5701;

/// LOD | Level of detail
pub const CUTSCENE_ACTION_SET_LOD_DEPRECATED: u32 = 5720;

/// Play Bink Movie | Which movie to play
pub const CUTSCENE_ACTION_PLAYMOVIE: u32 = 5740;

/// Gore | Gore level
pub const CUTSCENE_ACTION_SETGORE: u32 = 5750;

/// Name | Name
pub const MMH_NAME: u32 = 6000;

/// Material Object | Material Object
pub const MMH_MATERIAL_OBJECT: u32 = 6001;

/// Material Library | Material Library
pub const MMH_MATERIAL_LIBRARY: u32 = 6002;

/// Resource Name | Filename of the resource
pub const MMH_RESNAME: u32 = 6003;

/// ID | ID
pub const MMH_ID: u32 = 6004;

/// Mesh Data Name | MSH File
pub const MMH_MODEL_HIERARCHY_MODEL_DATA_NAME: u32 = 6005;

/// Mesh Group Name | Name
pub const MMH_MESH_GROUP_NAME: u32 = 6006;

/// Point Light Color | Point Light Light color
pub const MMH_NODE_POINT_LIGHT_COLOR: u32 = 6007;

/// Point Light Radius | Point Light Radius
pub const MMH_NODE_POINT_LIGHT_RADIUS: u32 = 6008;

/// Ambient Light Color | Ambient Light color
pub const MMH_NODE_AMBIENT_LIGHT_COLOR: u32 = 6010;

/// Emitter Birth Rate | Emitter Birth Rate
pub const MMH_NODE_EMITTER_BIRTH_RATE: u32 = 6011;

/// Emitter Life | Emitter Life
pub const MMH_NODE_EMITTER_LIFE: u32 = 6013;

/// Emitter Life Range | Emitter Life Range
pub const MMH_NODE_EMITTER_LIFE_RANGE: u32 = 6014;

/// Emitter Scale Range | Emitter Scale Range
pub const MMH_NODE_EMITTER_SCALE_RANGE: u32 = 6015;

/// Emitter Initial Speed | Emitter Initial Speed
pub const MMH_NODE_EMITTER_INITIAL_SPEED: u32 = 6016;

/// Emitter Acceleration | Emitter Acceleration
pub const MMH_NODE_EMITTER_ACCELERATION: u32 = 6018;

/// Emitter Inverse Movement Spread Update Delay | Inverse Emitter Movement Spread Update Delay
pub const MMH_NODE_INV_EMITTER_MOVEMENT_SPREAD_UPDATE_DELAY: u32 = 6022;

/// Emitter Bitflags | Emitter Bitflags
pub const MMH_NODE_EMITTER_OPTIONS_BITFLAGS: u32 = 6027;

/// Emitter Birthrate In Particles Per Meter flag | Emitter Birthrate In Particles Per Meter flag
pub const MMH_NODE_EMITTER_OPTIONS_BIRTHRATE_IN_PARTICLES_PER_METER: u32 = 6028;

/// Deprecated March 20/08
pub const MMH_NODE_EMITTER_OPTIONS_RANDOM_INITIAL_ROTATION: u32 = 6029;

/// Emitter Particles Affected By Wind flag | Emitter Particles Affected By Wind flag
pub const MMH_NODE_EMITTER_OPTIONS_PARTICLES_AFFECTED_BY_WIND: u32 = 6030;

/// Emitter Particles multiplier for gravity
pub const MMH_NODE_EMITTER_GRAVITY_MULTIPLIER: u32 = 6031;

/// Emitter Update Only When Visible flag | Emitter Update Only When Visible flag
pub const MMH_NODE_EMITTER_OPTIONS_UPDATE_ONLY_WHEN_VISIBLE: u32 = 6034;

/// Emitter Enable Particle Collisions flag | Emitter Enable Particle Collisions flag
pub const MMH_NODE_EMITTER_OPTIONS_ENABLE_PARTICLE_COLLISIONS: u32 = 6035;

/// Emitter Inherit Velocity Instead Of Position flag | Emitter Inherit Velocity Instead Of Position flag
pub const MMH_NODE_EMITTER_OPTIONS_INHERIT_VELOCITY_INSTEAD_OF_POSITION: u32 = 6036;

/// Age Map Count | Age Map Count
pub const MMH_NODE_AGE_MAP_COUNT: u32 = 6039;

/// Age Map Element Color | Age Map Element Color
pub const MMH_NODE_AGE_MAP_ELEMENT_COLOR: u32 = 6043;

/// Spawn Volume Options Spawn Within Volume | Spawn Volume Options Spawn Within Volume
pub const MMH_NODE_SPAWN_VOLUME_OPTIONS_SPAWN_WITHIN_VOLUME: u32 = 6045;

/// Spawn Volume Options Invert Spawn Volume Normals | Spawn Volume Options Invert Spawn Volume Normals
pub const MMH_NODE_SPAWN_VOLUME_OPTIONS_INVERT_SPAWN_VOLUME_NORMALS: u32 = 6046;

/// Translation | Translation
pub const MMH_TRANSLATION: u32 = 6047;

/// Rotation | Rotation
pub const MMH_ROTATION: u32 = 6048;

/// Attribute Name | Attribute Name
pub const MMH_ATTRIBUTE_NAME: u32 = 6049;

/// Attribute Source Name | Attribute Source Name
pub const MMH_ATTRIBUTE_SOURCE_NAME: u32 = 6050;

/// Export Tag Name | Export Tag Name
pub const MMH_EXPORT_TAG_NAME: u32 = 6051;

/// Export Name | Export Name
pub const MMH_EXPORT_EXPORT_NAME: u32 = 6052;

/// Export Controller Type | Export Controller Type
pub const MMH_EXPORT_CONTROLLER_TYPE: u32 = 6053;

/// Bounding Box Min | Bounding Box Min
pub const MMH_BOUNDING_BOX_MIN: u32 = 6054;

/// Bounding Box Max | Bounding Box Max
pub const MMH_BOUNDING_BOX_MAX: u32 = 6055;

/// Collision Object Type | Collision Object Type
pub const MMH_NODE_COLLISION_OBJ_TYPE: u32 = 6057;

/// Shape Type | Shape Type
pub const MMH_SHAPE_TYPE: u32 = 6058;

/// Shape PMat Name | Shape PMat Name
pub const MMH_SHAPE_PMAT_NAME: u32 = 6059;

/// Shape Rotation | Shape Rotation
pub const MMH_SHAPE_ROTATION: u32 = 6060;

/// Shape Position | Shape Position
pub const MMH_SHAPE_POSITION: u32 = 6061;

/// Shape Collision Mask Static Geometry flag | Shape Collision Mask Static Geometry flag
pub const MMH_SHAPE_COLLISION_MASK_STATIC_GEOMETRY: u32 = 6069;

/// Shape Collision Mask Non-Walkable flag | Shape Collision Mask Non-Walkable flag
pub const MMH_SHAPE_COLLISION_MASK_NONWALKABLE: u32 = 6070;

/// Shape Box Dim | Shape Box Dim
pub const MMH_SHAPE_BOX_DIM: u32 = 6071;

/// Shape Radius | Shape Radius
pub const MMH_SHAPE_RADIUS: u32 = 6072;

/// Shape Height | Shape Height
pub const MMH_SHAPE_HEIGHT: u32 = 6073;

/// Mesh Shape Flags | Mesh Shape Flags
pub const MMH_SHAPE_MESH_SHAPE_FLAGS: u32 = 6074;

/// Joint Target | Joint Target
pub const MMH_JOINT_TARGET: u32 = 6078;

/// Joint Type | Joint Type
pub const MMH_JOINT_TYPE: u32 = 6079;

/// Joint Joint Type | Joint Joint Type
pub const MMH_JOINT_TYPE_STRUCT: u32 = 6080;

/// Joint Local Normal 1 | Joint Local Normal 1
pub const MMH_JOINT_LOCAL_NORMAL_1: u32 = 6081;

/// Joint Local Normal 2 | Joint Local Normal 2
pub const MMH_JOINT_LOCAL_NORMAL_2: u32 = 6082;

/// Joint Local Anchor 1 | Joint Local Anchor 1
pub const MMH_JOINT_LOCAL_ANCHOR_1: u32 = 6083;

/// Joint Local Anchor 2 | Joint Local Anchor 2
pub const MMH_JOINT_LOCAL_ANCHOR_2: u32 = 6084;

/// Joint Local Axis 1 | Joint Local Axis 1
pub const MMH_JOINT_LOCAL_AXIS_1: u32 = 6085;

/// Joint Local Axis 2 | Joint Local Axis 2
pub const MMH_JOINT_LOCAL_AXIS_2: u32 = 6086;

/// Joint Max Force | Joint Max Force
pub const MMH_JOINT_MAX_FORCE: u32 = 6087;

/// Joint Max Torque | Joint Max Torque
pub const MMH_JOINT_MAX_TORQUE: u32 = 6088;

/// Joint Collision Enabled | Joint Collision Enabled
pub const MMH_JOINT_COLLISION_ENABLED: u32 = 6089;

/// Spherical Swing Axis | Spherical Swing Axis
pub const MMH_JOINT_SPHERICAL_SWING_AXIS: u32 = 6090;

/// Spherical Swing Limit | Spherical Swing Limit
pub const MMH_JOINT_SPHERICAL_SWING_LIMIT: u32 = 6094;

/// Revolute Limit Low | Revolute Limit Low
pub const MMH_JOINT_REVOLUTE_LIMIT_LOW: u32 = 6100;

/// Revolute Limit High | Revolute Limit High
pub const MMH_JOINT_REVOLUTE_LIMIT_HIGH: u32 = 6101;

/// Revolute Spring | Revolute Spring
pub const MMH_JOINT_REVOLUTE_SPRING: u32 = 6105;

/// Revolute Max Force | Revolute Max Force
pub const MMH_JOINT_REVOLUTE_MOTOR_MAX_FORCE: u32 = 6107;

/// Revolute Free Spin | Revolute Free Spin
pub const MMH_JOINT_REVOLUTE_MOTOR_FREE_SPIN: u32 = 6108;

/// Revolute Flags | Revolute Flags
pub const MMH_JOINT_REVOLUTE_REVOLUTE_FLAGS: u32 = 6109;

/// Distance Min Distance | Distance Min Distance
pub const MMH_JOINT_DISTANCE_MIN_DISTANCE: u32 = 6110;

/// Distance Max Distance | Distance Max Distance
pub const MMH_JOINT_DISTANCE_MAX_DISTANCE: u32 = 6111;

/// Distance Spring | Distance Spring
pub const MMH_JOINT_DISTANCE_SPRING: u32 = 6112;

/// Distance Flags | Distance Flags
pub const MMH_JOINT_DISTANCE_DISTANCE_FLAGS: u32 = 6113;

/// Pulley 1 | Pulley 1
pub const MMH_JOINT_PULLEY_PULLEY_1: u32 = 6114;

/// Pulley 2 | Pulley 2
pub const MMH_JOINT_PULLEY_PULLEY_2: u32 = 6115;

/// Pulley Distance | Pulley Distance
pub const MMH_JOINT_PULLEY_DISTANCE: u32 = 6116;

/// Pulley Stiffness | Pulley Stiffness
pub const MMH_JOINT_PULLEY_STIFFNESS: u32 = 6117;

/// Pulley Ratio | Pulley Ratio
pub const MMH_JOINT_PULLEY_RATIO: u32 = 6118;

/// Pulley Flags | Pulley Flags
pub const MMH_JOINT_PULLEY_PULLEY_FLAGS: u32 = 6122;

/// 6DOF X Motion | 6DOF X Motion
pub const MMH_JOINT_6DOF_X_MOTION: u32 = 6123;

/// 6DOF Y Motion | 6DOF Y Motion
pub const MMH_JOINT_6DOF_Y_MOTION: u32 = 6124;

/// 6DOF Z Motion | 6DOF Z Motion
pub const MMH_JOINT_6DOF_Z_MOTION: u32 = 6125;

/// 6DOF Swing 1 Motion | 6DOF Swing 1 Motion
pub const MMH_JOINT_6DOF_SWING_1_MOTION: u32 = 6126;

/// 6DOF Swing 2 Motion | 6DOF Swing 2 Motion
pub const MMH_JOINT_6DOF_SWING_2_MOTION: u32 = 6127;

/// 6DOF Twist Motion | 6DOF Twist Motion
pub const MMH_JOINT_6DOF_TWIST_MOTION: u32 = 6128;

/// 6DOF Linear Limit | 6DOF Linear Limit
pub const MMH_JOINT_6DOF_LINEAR_LIMIT: u32 = 6129;

/// 6DOF Swing 1 Limit | 6DOF Swing 1 Limit
pub const MMH_JOINT_6DOF_SWING_1_LIMIT: u32 = 6130;

/// 6DOF Swing 2 Limit | 6DOF Swing 2 Limit
pub const MMH_JOINT_6DOF_SWING_2_LIMIT: u32 = 6131;

/// 6DOF Twist Limit Low | 6DOF Twist Limit Low
pub const MMH_JOINT_6DOF_TWIST_LIMIT_LOW: u32 = 6132;

/// 6DOF Twist Limit High | 6DOF Twist Limit High
pub const MMH_JOINT_6DOF_TWIST_LIMIT_HIGH: u32 = 6133;

/// 6DOF Drive Position | 6DOF Drive Position
pub const MMH_JOINT_6DOF_DRIVE_POSITION: u32 = 6159;

/// 6DOF Gear Ratio | 6DOF Gear Ratio
pub const MMH_JOINT_6DOF_GEAR_RATIO: u32 = 6164;

/// 6DOF Projection Mode | 6DOF Projection Mode
pub const MMH_JOINT_6DOF_PROJECTION_MODE: u32 = 6165;

/// 6DOF D6 Flags | 6DOF D6 Flags
pub const MMH_JOINT_6DOF_D6_FLAGS: u32 = 6166;

/// Data Semantic | Data Semantic
pub const MMH_DATA_SEMANTIC: u32 = 6167;

/// Data Is Index Stream | Data Is Index Stream
pub const MMH_DATA_IS_INDEX_STREAM: u32 = 6168;

/// Data Type/Index Type | Data Type/Index Type
pub const MMH_DATA_TYPE: u32 = 6169;

/// Data Bitflags | Data Bitflags
pub const MMH_DATA_BITFLAGS: u32 = 6170;

/// Data Data Looping flag | Data Data Looping flag
pub const MMH_DATA_LOOPING: u32 = 6171;

/// Data Instanced flag | Data Instanced flag
pub const MMH_DATA_INSTANCED: u32 = 6172;

/// Data Count | Data Count
pub const MMH_DATA_COUNT: u32 = 6173;

/// Data Primitive Type | Data Primitive Type
pub const MMH_DATA_PRIMITIVE_TYPE: u32 = 6174;

/// Data Frequency | Data Frequency
pub const MMH_DATA_FREQUENCY: u32 = 6175;

/// Runtime Shadow Casting | Runtime Shadow Casting
pub const MMH_MESH_CAST_RUNTIME_SHADOW: u32 = 6176;

/// Cast Baked Shadow | Cast Baked Shadow
pub const MMH_MESH_CAST_BAKED_SHADOW: u32 = 6177;

/// Shape Collision Mask Effects
pub const MMH_SHAPE_COLLISION_MASK_EFFECTS: u32 = 6178;

/// Shape Collision Mask Waypoints
pub const MMH_SHAPE_COLLISION_MASK_WAYPOINTS: u32 = 6179;

/// Flipbook FPS | Flipbook FPS
pub const MMH_FLIPBOOK_FRAMES_PER_SECOND: u32 = 6180;

/// Flipbook Rows | Flipbook Rows
pub const MMH_FLIPBOOK_ROWS: u32 = 6181;

/// Flipbook Columns | Flipbook Columns
pub const MMH_FLIPBOOK_COLUMNS: u32 = 6182;

/// Emitter Target Name | Emitter Target Name
pub const MMH_EMITTER_TARGET_NAME: u32 = 6184;

/// Emitter Target Radius | Emitter Target Radius
pub const MMH_EMITTER_TARGET_RADIUS: u32 = 6186;

/// Emitter Spawn Direction Tracks Target | Emitter Spawn Direction Tracks Target
pub const MMH_EMITTER_SPAWN_DIRECTION_TRACKS_TARGET: u32 = 6187;

/// Emitter flipbook type
pub const MMH_EMITTER_FLIPBOOK_TYPE: u32 = 6189;

/// Cut Away | Cut Away
pub const MMH_MESH_CUT_AWAY: u32 = 6193;

/// Punch Through | Punch Through
pub const MMH_MESH_PUNCH_THROUGH: u32 = 6194;

/// Cloth thickness | Cloth thickness
pub const MMH_CLOTH_THICKNESS: u32 = 6195;

/// Cloth density | Cloth density
pub const MMH_CLOTH_DENSITY: u32 = 6196;

/// Cloth bending stiffness | Cloth bending stiffness
pub const MMH_CLOTH_BENDING_STIFFNESS: u32 = 6197;

/// Cloth friction | Cloth friction
pub const MMH_CLOTH_FRICTION: u32 = 6200;

/// Cloth pressure | Cloth pressure
pub const MMH_CLOTH_PRESSURE: u32 = 6201;

/// Cloth tear factor | Cloth tear factor
pub const MMH_CLOTH_TEAR_FACTOR: u32 = 6202;

/// Cloth collisions response coefficient | Cloth collisions response coefficient
pub const MMH_CLOTH_COLLISION_RESPONSE_COEFFICIENT: u32 = 6203;

/// Cloth attachment response coefficient | Cloth attachment response coefficient
pub const MMH_CLOTH_ATTACHMENT_RESPONSE_COEFFICIENT: u32 = 6204;

/// Cloth solver iterations | Cloth solver iterations
pub const MMH_CLOTH_SOLVER_ITERATIONS: u32 = 6206;

/// Cloth wake up counter | Cloth wake up counter
pub const MMH_CLOTH_WAKE_UP_COUNTER: u32 = 6208;

/// Cloth flag bitflags | Cloth flag bitflags
pub const MMH_CLOTH_FLAG_BITFLAGS: u32 = 6210;

/// Cloth flag pressure | Cloth flag pressure
pub const MMH_CLOTH_FLAG_PRESSURE: u32 = 6211;

/// Cloth flag static | Cloth flag static
pub const MMH_CLOTH_FLAG_STATIC: u32 = 6212;

/// Cloth flag gravity | Cloth flag gravity
pub const MMH_CLOTH_FLAG_GRAVITY: u32 = 6216;

/// Cloth flag bending | Cloth flag bending
pub const MMH_CLOTH_FLAG_BENDING: u32 = 6217;

/// Cloth flag damping | Cloth flag damping
pub const MMH_CLOTH_FLAG_DAMPING: u32 = 6219;

/// Cloth flag tearable | Cloth flag tearable
pub const MMH_CLOTH_FLAG_TEARABLE: u32 = 6222;

/// Cloth flag hardware | Cloth flag hardware
pub const MMH_CLOTH_FLAG_HARDWARE: u32 = 6223;

/// Cloth flag COM damping | Cloth flag COM damping
pub const MMH_CLOTH_FLAG_COMDAMPING: u32 = 6224;

/// Cloth attachment type | Cloth attachment type
pub const MMH_CLOTH_ATTACHMENT_TYPE: u32 = 6225;

/// Cloth attachment flag two way attachment | Cloth attachment flag two way attachment
pub const MMH_CLOTH_ATTACHMENT_FLAG_TWO_WAY_ATTACHMENT: u32 = 6227;

/// Cloth attachment flag tearable attachment | Cloth attachment flag tearable attachment
pub const MMH_CLOTH_ATTACHMENT_FLAG_TEARABLE_ATTACHMENT: u32 = 6228;

/// Mesh Group for Cloth | Mesh Group for Cloth
pub const MMH_CLOTH_MESH_GROUP_STRUCT: u32 = 6233;

/// Emitter type | Emitter type
pub const MMH_NODE_EMITTER_TYPE: u32 = 6234;

/// Hook ID | Hook ID
pub const MMH_NODE_CRUST_HOOK_ID: u32 = 6235;

/// DEPRECATED
pub const MMH_COLLISION_OBJECT_VOLUME: u32 = 6236;

/// DEPRECATED
pub const MMH_OBJECT_VOLUME: u32 = 6237;

/// Exported Tag Type | Exported Tag Type
pub const MMH_EXPORT_TAG_VARIABLE_TYPE: u32 = 6238;

/// Is Physics Emitter | Is Physics Emitter
pub const MMH_EMITTER_IS_PHYSICS_EMITTER: u32 = 6239;

/// Physics Shape Volume | Physics Shape Volume
pub const MMH_SHAPE_VOLUME: u32 = 6240;

/// Shape Name | Shape Name
pub const MMH_SHAPE_NAME: u32 = 6241;

/// Snap Position | Vector3 position of a snap point
pub const MMH_SNAP_POSITION: u32 = 6242;

/// Allow Emitter Spawn | Allow Emitter Spawn
pub const MMH_SHAPE_ALLOW_EMITTER_SPAWN: u32 = 6244;

/// Collision Group | Collision Group
pub const MMH_COLLISION_GROUP: u32 = 6245;

/// Point Light Intensity Variation | Point Light Intensity Variation for flickering lights
pub const MMH_NODE_POINT_LIGHT_INTENSITY_VARIATION: u32 = 6249;

/// Point Light Intensity Period | Point Light Intensity Period for flickering lights
pub const MMH_NODE_POINT_LIGHT_INTENSITY_PERIOD: u32 = 6250;

/// Point Light Intensity Period delta | Point Light Intensity Period delta for flickering lights
pub const MMH_NODE_POINT_LIGHT_INTENSITY_PERIOD_DELTA: u32 = 6251;

/// Fadeable physics part
pub const MMH_SHAPE_FADEABLE: u32 = 6252;

/// Bone index | Index of this bone into the object's bone array
pub const MMH_BONE_INDEX: u32 = 6254;

/// TRUE if Wind enabled for cloth
pub const MMH_CLOTH_WIND_ENABLED: u32 = 6257;

/// World or local space of wind direction
pub const MMH_CLOTH_WIND_SPACE: u32 = 6258;

/// Wind direction for cloth
pub const MMH_CLOTH_WIND_DIRECTION: u32 = 6259;

/// The response of the cloth on wind per second
pub const MMH_CLOTH_WIND_RESPONSE: u32 = 6260;

/// The wind strength
pub const MMH_CLOTH_WIND_STRENGTH: u32 = 6262;

/// The gusting min strength
pub const MMH_CLOTH_WIND_GUST_MIN_STRENGTH: u32 = 6263;

/// The gusting max strength
pub const MMH_CLOTH_WIND_GUST_MAX_STRENGTH: u32 = 6264;

/// The gusting min duration
pub const MMH_CLOTH_WIND_GUST_MIN_DURATION: u32 = 6265;

/// The gusting max strength
pub const MMH_CLOTH_WIND_GUST_MAX_DURATION: u32 = 6266;

/// The minimum interval between two gusts
pub const MMH_CLOTH_WIND_GUST_MIN_INTERVAL: u32 = 6267;

/// The maximum interval between two gusts
pub const MMH_CLOTH_WIND_GUST_MAX_INTERVAL: u32 = 6268;

/// The parameter which shows how big is difference between wind and gusting direction
pub const MMH_CLOTH_WIND_GUST_DIR_CHANGE: u32 = 6269;

/// Export Controller Index| Index of this controller in the controller array (they are sorted by name)
pub const MMH_EXPORT_CONTROLLER_INDEX: u32 = 6274;

/// Number of exports | Number of controllers exported (for animation) on this model.
pub const MMH_TOTAL_EXPORTS: u32 = 6275;

/// Shape group flag
pub const MMH_SHAPE_COLLISION_MASK_WATER: u32 = 6277;

/// Scale of object node
pub const MMH_SCALE: u32 = 6278;

pub const MMH_NODE_EMITTER_AGEMAP_COLOR_MULTIPLIER: u32 = 6279;

pub const MMH_NODE_EMITTER_AGEMAP_SCALEX_MULTIPLIER: u32 = 6280;

pub const MMH_NODE_EMITTER_AGEMAP_SCALEY_MULTIPLIER: u32 = 6281;

pub const MMH_NODE_EMITTER_OPTIONS_BOUNCINESS: u32 = 6282;

pub const MMH_NODE_EMITTER_OPTIONS_FRICTION: u32 = 6283;

pub const MMH_NODE_EMITTER_MESH_PARTICLE_MODELNAME: u32 = 6284;

/// type of spawn volume to use
pub const MMH_NODE_SPAWN_VOLUME_TYPE: u32 = 6285;

/// procedural spawn volume radius
pub const MMH_NODE_SPAWN_VOLUME_RADIUS: u32 = 6286;

pub const MMH_NODE_SPAWN_VOLUME_CYLINDER_LENGTH: u32 = 6287;

pub const MMH_NODE_SPAWN_VOLUME_CYLINDER_AXIS: u32 = 6288;

pub const MMH_NODE_SPAWN_VOLUME_BOX_MIN: u32 = 6289;

pub const MMH_NODE_SPAWN_VOLUME_BOX_MAX: u32 = 6290;

/// Shape group flag
pub const MMH_SHAPE_COLLISION_MASK_TERRAIN_WALL: u32 = 6295;

/// Analogous to the GFF_LIGHT_AFFECT_DOMAIN tag, as it is read from an mmh file.
pub const MMH_NODE_LIGHT_AFFECT_DOMAIN: u32 = 6296;

/// vertex format for emitters
pub const MMN_NODE_EMITTER_VERTEX_FORMAT: u32 = 6297;

/// additional acceleration (m_vWorldAxisAcceleration) is actually in object space
pub const MMH_NODE_EMITTER_OPTIONS_OBJECT_SPACE_ACCELERATION: u32 = 6298;

/// particle initial rotation value
pub const MMH_NODE_EMITTER_INITIAL_ROTATION: u32 = 6299;

/// particle initial rotation range
pub const MMH_NODE_EMITTER_INITIAL_ROTATION_RANGE: u32 = 6300;

/// Receive Shadow | Receive Shadow
pub const MMH_MESH_RECEIVE_BAKED_SHADOW: u32 = 6301;

pub const MMH_NODE_EMITTER_MESH_PARTICLE_UP_AXIS: u32 = 6302;

pub const MMH_NODE_EMITTER_MESH_PARTICLE_ROLL_AXIS: u32 = 6303;

/// Receive Shadow | Receive Shadow
pub const MMH_MESH_RECEIVE_RUNTIME_SHADOW: u32 = 6304;

/// Walkable flag
pub const MMH_SHAPE_COLLISION_MASK_WALKABLE: u32 = 6305;

/// list of referenced meshes
pub const MMH_MODEL_MESH_NAME_LIST: u32 = 6306;

/// which referenced mesh to look in
pub const MMH_NODE_MESH_NAME: u32 = 6307;

/// render after distortion effects
pub const MMH_NODE_EMITTER_IGNORE_DISTORTION: u32 = 6309;

pub const MMH_NODE_EMITTER_SPLATPARAMS_WIDTH: u32 = 6310;

pub const MMH_NODE_EMITTER_SPLATPARAMS_HEIGHT: u32 = 6311;

pub const MMH_NODE_EMITTER_SPLATPARAMS_NUMSAMPLES_WIDTH: u32 = 6312;

pub const MMH_NODE_EMITTER_SPLATPARAMS_NUMSAMPLES_HEIGHT: u32 = 6313;

pub const MMH_NODE_EMITTER_SPLATPARAMS_ORIENTATION_RANGE: u32 = 6314;

pub const MMH_NODE_EMITTER_SPLATPARAMS_LIFE: u32 = 6315;

pub const MMH_NODE_EMITTER_SPLATPARAMS_FLIPBOOK_TYPE: u32 = 6316;

pub const MMH_NODE_EMITTER_SPLATPARAMS_FLIPBOOK_FRAMES_PER_SECOND: u32 = 6317;

pub const MMH_NODE_EMITTER_SPLATPARAMS_FLIPBOOK_ROWS: u32 = 6318;

pub const MMH_NODE_EMITTER_SPLATPARAMS_FLIPBOOK_COLUMNS: u32 = 6319;

pub const MMH_NODE_EMITTER_SPLATPARAMS_FLIPBOOK_RANDOM_START_FRAME: u32 = 6320;

pub const MMH_NODE_EMITTER_CAN_PARTICLES_SPLAT: u32 = 6321;

pub const MMH_NODE_EMITTER_SPLATPARAMS_AGE_MAP_ELEMENT_PERCENT_LIFE_ELAPSED: u32 = 6322;

pub const MMH_NODE_EMITTER_LOD: u32 = 6323;

pub const MMH_NODE_EMITTER_SPLATPARAMS_MATERIALNAME: u32 = 6324;

pub const MMH_NODE_EMITTER_USER_PARAM_NAME: u32 = 6325;

pub const MMH_REMOTE_MATERIAL_DECAL_NAME: u32 = 6327;

pub const MMH_REMOTE_MATERIAL_FRESNEL_FALLOFF: u32 = 6328;

pub const MMH_REMOTE_MATERIAL_INVERT_FRESNEL: u32 = 6329;

pub const MMH_NODE_SOUND_MATERIAL: u32 = 6330;

pub const MMH_REMOTE_MATERIAL_ALPHA: u32 = 6331;

pub const MMH_REMOTE_MATERIAL_TINT: u32 = 6332;

pub const MMH_EMITTER_PRESIMULATE_TIME: u32 = 6333;

pub const MMH_MESH_IS_VFX_MESH: u32 = 6334;

pub const MMH_MESH_MATERIAL_COLOR: u32 = 6335;

pub const MMH_LIGHTPROBE_IRRADIANCE_RED: u32 = 6336;

pub const MMH_LIGHTPROBE_IRRADIANCE_GREEN: u32 = 6337;

pub const MMH_LIGHTPROBE_IRRADIANCE_BLUE: u32 = 6338;

/// Can Be Occluded |bool Whether or not light can be occluded at runtime (based on light subsets).
pub const MMH_LIGHT_CAN_BE_OCCLUDED: u32 = 6339;

/// can use designer variation tint color
pub const MMH_USE_VARIATION_TINT: u32 = 6340;

/// Shape Type Struct | Generic Struct Reference
pub const MMH_SHAPE_TYPE_STRUCT: u32 = 6998;

/// Children | Generic Struct Reference List
pub const MMH_CHILDREN: u32 = 6999;

/// Terrain Name | string.  Name of terrain.
pub const TERRAIN_VERSION: u32 = 7000;

/// Base Rows | Number of rows in a sector.
pub const TERRAIN_BASE_ROWS: u32 = 7001;

/// Base Columns | Number of columns in a sector.
pub const TERRAIN_BASE_COLUMNS: u32 = 7002;

/// Length Units | Length of a sector.
pub const TERRAIN_LENGTH_UNITS: u32 = 7003;

/// Width Units | Width of a sector.
pub const TERRAIN_WIDTH_UNITS: u32 = 7004;

/// Area Rows | Number of row sectors.
pub const TERRAIN_SECTOR_ROWS: u32 = 7005;

/// Area Columns | Number of column sectors.
pub const TERRAIN_SECTOR_COLUMNS: u32 = 7006;

/// Tessellation | Tessellation level.
pub const TERRAIN_TESSELLATION: u32 = 7007;

/// Sector ID | ID of a sector.
pub const TERRAIN_SECTOR_ID: u32 = 7008;

/// Sector List | List of sectors
pub const TERRAIN_SECTOR_LIST: u32 = 7009;

/// Face ID | ID of a face.
pub const TERRAIN_MESHFACE_ID: u32 = 7010;

/// Face List | List of faces.
pub const TERRAIN_MESHFACE_LIST: u32 = 7011;

/// Edge ID | ID of an edge.
pub const TERRAIN_MESHEDGE_ID: u32 = 7012;

/// Edge Start Vertex | Starting vertex for an edge.
pub const TERRAIN_MESHEDGE_START_VERTEX: u32 = 7013;

/// Edge List | List of edges.
pub const TERRAIN_MESHEDGE_LIST: u32 = 7016;

/// SubEdge ID | ID of a subedge.
pub const TERRAIN_SUBEDGE_ID: u32 = 7017;

/// Vertex ID | ID of a vertex.
pub const TERRAIN_MESHVERTEX_ID: u32 = 7018;

/// Vertex Position | vector3 position of a vertex.
pub const TERRAIN_MESHVERTEX_POSITION: u32 = 7019;

/// Vertex Level | int32 level of a vertex.
pub const TERRAIN_MESHVERTEX_LEVEL: u32 = 7020;

/// Vertex List | List of vertices.
pub const TERRAIN_MESHVERTEX_LIST: u32 = 7023;

/// Element ID Sector | Sector for the Element ID.
pub const TERRAIN_ELEMENT_ID_SECTOR: u32 = 7025;

/// Material Name | string.  Name of material.
pub const TERRAIN_MATERIAL_VALUE: u32 = 7026;

/// Material List | List of materials.
pub const TERRAIN_MATERIAL_LIST: u32 = 7027;

/// Area Information | Information about the area.
pub const TERRAIN_AREA_INFORMATION: u32 = 7028;

/// Vertex U | U texture coordinate of a vertex.
pub const TERRAIN_VERTEX_U: u32 = 7029;

/// Vertex V | U texture coordinate of a vertex.
pub const TERRAIN_VERTEX_V: u32 = 7030;

/// Map Vertex ID | ID of a map vertex.
pub const TERRAIN_MAPVERTEX_ID: u32 = 7037;

/// Map Vertex UVW | vector3 position of a map vertex.
pub const TERRAIN_MAPVERTEX_UVW: u32 = 7038;

/// Map Vertex List | List of map vertices.
pub const TERRAIN_MAPVERTEX_LIST: u32 = 7039;

/// Map Edge ID | ID of a map edge.
pub const TERRAIN_MAPEDGE_ID: u32 = 7040;

/// Map Edge List | List of map edges.
pub const TERRAIN_MAPEDGE_LIST: u32 = 7042;

/// Map Face ID | ID of a map face.
pub const TERRAIN_MAPFACE_ID: u32 = 7043;

/// Map Face Layer | Layer of a map face.
pub const TERRAIN_MAPFACE_LAYER: u32 = 7044;

/// Map Face List | List of map faces.
pub const TERRAIN_MAPFACE_LIST: u32 = 7045;

/// Blend Weight | Ordered lookup ID for material
pub const TERRAIN_BLENDWEIGHT_MATID: u32 = 7047;

/// Blend Page | ID of page.
pub const TERRAIN_BLENDPAGE_ID: u32 = 7050;

/// Blend Page | Width in texels.
pub const TERRAIN_BLENDPAGE_WIDTH: u32 = 7051;

/// Blend Page | Height in texels.
pub const TERRAIN_BLENDPAGE_HEIGHT: u32 = 7052;

/// Blend Page | Texels linear array.
pub const TERRAIN_BLENDPAGE_TEXEL_LIST: u32 = 7053;

/// Blend Page List | List of blend pages.
pub const TERRAIN_BLENDPAGE_LIST: u32 = 7054;

/// Mesh | struct "MESH".
pub const TERRAIN_MESH: u32 = 7055;

/// Palette | Material palette.
pub const TERRAIN_PALETTE: u32 = 7056;

/// Blend Weight | Byte weight list
pub const TERRAIN_BLENDTEXEL_BYTEWEIGHTLIST: u32 = 7057;

/// Mesh Name | Name of mesh.
pub const TERRAIN_MESH_NAME: u32 = 7058;

/// Palette Name | Name of palette.
pub const TERRAIN_PALETTE_NAME: u32 = 7059;

/// Material | Material in palette.
pub const TERRAIN_MATERIAL: u32 = 7060;

/// Material Name | ID of a material.
pub const TERRAIN_MATERIAL_ID: u32 = 7061;

/// Material Name | Name of a material.
pub const TERRAIN_MATERIAL_NAME: u32 = 7062;

/// Material Scale | Scale applied to material textures.
pub const TERRAIN_MATERIAL_SCALE: u32 = 7063;

/// Diffuse Name | ResName of a texture.
pub const TERRAIN_MATERIAL_DIFFUSE_NAME: u32 = 7064;

/// Normal Name | ResName of a texture.
pub const TERRAIN_MATERIAL_NORMAL_NAME: u32 = 7065;

/// Specular Name | ResName of a texture.
pub const TERRAIN_MATERIAL_SPECUALAR_NAME: u32 = 7066;

/// Heightmap Name | ResName of a texture.
pub const TERRAIN_MATERIAL_HEIGHTMAP_NAME: u32 = 7067;

/// Vertex Constraint #A.
pub const TERRAIN_MESHVERTEX_CONSTRAINT_A: u32 = 7071;

/// Vertex Constraint #A.
pub const TERRAIN_MESHVERTEX_CONSTRAINT_B: u32 = 7072;

/// Edge Subedge List. | List of subedges for an edge.
pub const TERRAIN_MESHEDGE_SUBEDGE_A: u32 = 7073;

/// Edge Subedge List. | List of subedges for an edge.
pub const TERRAIN_MESHEDGE_SUBEDGE_B: u32 = 7074;

/// Blend texel unique ID.
pub const TERRAIN_BLENDTEXEL_ID: u32 = 7075;

/// Material sound property | Terrain material sound property
pub const TERRAIN_SOUND_DATA: u32 = 7076;

/// Specular Color | Per material specular color.
pub const TERRAIN_MATERIAL_SPECULAR_COLOR: u32 = 7077;

pub const WATER_INFORMATION: u32 = 7900;

pub const WATER_VERSION: u32 = 7901;

pub const WATER_ID: u32 = 7902;

pub const WATER_VERTEX_LIST: u32 = 7903;

pub const WATER_VERTEX_POSITION: u32 = 7904;

pub const WATER_VERTEX_NORMAL: u32 = 7905;

pub const WATER_VERTEX_UVW: u32 = 7906;

pub const WATER_VERTEX_COLOR: u32 = 7907;

pub const WATER_VERTEX_INDEX_LIST: u32 = 7908;

/// Vertex Size | The size of each vertex in Bytes
pub const MESH_CHUNK_VERTEXSIZE: u32 = 8000;

/// Index Count | The number of indices in this mesh chunk
pub const MESH_CHUNK_INDEXCOUNT: u32 = 8002;

/// Primitive Type | The primitive type
pub const MESH_CHUNK_PRIMITIVETYPE: u32 = 8003;

/// Base Vertex Index | The index into the vertex list at which to start indexing
pub const MESH_CHUNK_BASEVERTEXINDEX: u32 = 8005;

/// Vertex Offset | The offset into the list (plus the base) where this chunk's vertices are
pub const MESH_CHUNK_VERTEXOFFSET: u32 = 8006;

/// Min Index | The lowest index in the list of indices
pub const MESH_CHUNK_MININDEX: u32 = 8007;

/// Start Index | The which index in the stream is the first index used for this chunk
pub const MESH_CHUNK_STARTINDEX: u32 = 8009;

/// Has Instance Geometry | This field is true (not zero) if this mesh represents instanced geometry
pub const MESH_CHUNK_HASINSTGEOM: u32 = 8010;

/// Vertex Size | The size of the vertices in the stream
pub const MESH_STREAM_VERTEXSIZE: u32 = 8012;

/// Vertex Count | The number of vertices in the stream
pub const MESH_STREAM_VERTEXCOUNT: u32 = 8013;

/// Frequency | The frequency of the stream
pub const MESH_STREAM_FREQUENCY: u32 = 8014;

/// Looping | Whether or not the stream loops
pub const MESH_STREAM_LOOPING: u32 = 8015;

/// Instanced | Whether or not the stream is instanced
pub const MESH_STREAM_INSTANCED: u32 = 8016;

/// Bounding Box Min | The min point of the bounding box
pub const MESH_BOUNDS_BOXMIN: u32 = 8017;

/// Bounding Box Max | The max point of the bounding box
pub const MESH_BOUNDS_BOXMAX: u32 = 8018;

/// Bounding Sphere | The bounding sphere
pub const MESH_BOUNDS_SPHERE: u32 = 8019;

/// Bounds | Holds bounding info
pub const MESH_CHUNK_BOUNDS: u32 = 8020;

/// Chunks | List of chunks for this mesh
pub const MESH_CHUNKS: u32 = 8021;

/// IndexData | The index data that composes this mesh
pub const MESH_INDEXDATA: u32 = 8023;

/// Vertex Data | The vertex data for this stream
pub const MESH_STREAM_VERTEXDATA: u32 = 8024;

/// Stream | Which stream this is using
pub const MESH_VERTEXDECLARATOR_STREAM: u32 = 8026;

/// Offset | THe offset into the data for this stream
pub const MESH_VERTEXDECLARATOR_OFFSET: u32 = 8027;

/// Usage | The usage is the semantic meaning of the attribute being described by this decl
pub const MESH_VERTEXDECLARATOR_USAGE: u32 = 8029;

/// Usage Index | If there is more than one of the same usage this value describes which one it is
pub const MESH_VERTEXDECLARATOR_USAGEINDEX: u32 = 8030;

/// Index Format | The data format of the indices,
pub const MESH_INDEXFORMAT: u32 = 8032;

/// Int8 | If the mesh stream data is used for instancing
pub const MESH_INSTANCED_STREAM: u32 = 8033;

/// Int32 | Number of instances in a binary mesh chunk
pub const MESH_CHUNK_INSTANCES_COUNT: u32 = 8034;

/// Blend node name | Blend node name
pub const AC_NODE_NAME: u32 = 9000;

/// Edge start socket ID | Edge start socket ID
pub const AC_EDGE_START_ID: u32 = 9001;

/// Edge end socket ID | Edge end socket ID
pub const AC_EDGE_END_ID: u32 = 9002;

/// Caption | Caption
pub const AC_CAPTION: u32 = 9003;

/// Node's inputs and outputs | Socket list
pub const AC_NODE_SOCKET_LIST: u32 = 9004;

/// Is this an output | Input or output?
pub const AC_SOCKET_IS_OUTPUT: u32 = 9005;

/// Index in the image list | Image index
pub const AC_NODE_IMAGE: u32 = 9006;

/// Connectors in the graph | Connectors
pub const AC_EDGE_LIST: u32 = 9007;

/// Blend nodes | blend nodes
pub const AC_NODE_LIST: u32 = 9008;

/// Blend node background colour | Background colour
pub const AC_NODE_COLOUR: u32 = 9009;

/// Animation for stream node | Animation
pub const AC_NODE_ANIMATION: u32 = 9010;

/// Value of the control point | Control point value
pub const AC_CURVE_CONTROL_POINT_VALUE: u32 = 9013;

/// Name of the model to be animated | Model name
pub const AC_MODEL_NAME: u32 = 9014;

/// List of events | List of events
pub const AC_EVENT_LIST: u32 = 9015;

/// Time at which the event fires | Event fire time
pub const AC_EVENT_TIME: u32 = 9016;

/// Event ID | Event ID
pub const AC_EVENT_ID: u32 = 9017;

/// Various flags for a given file | Flags
pub const AC_FLAGS: u32 = 9019;

/// Transition animation name | Animation name
pub const AC_TRANS_ANIM_NAME: u32 = 9020;

/// Animation start time in transitions | Trans start time
pub const AC_TRANS_ANIM_START: u32 = 9021;

/// Animation length for transitions | Animation length
pub const AC_TRANS_ANIM_LENGTH: u32 = 9022;

/// List of animation structures for transitions (to/from) | Transition animations
pub const AC_TRANS_TRACK_LIST: u32 = 9023;

/// List of transitions in the ACB file | Transitions
pub const AC_TRANSITION_LIST: u32 = 9024;

/// Length of the transition | Transition length
pub const AC_TRANS_LENGTH: u32 = 9025;

/// List of animation structures | Blend group animations
pub const AC_BLENDGROUP_ANIM_LIST: u32 = 9100;

/// List of blend groups | Blend groups
pub const AC_BLEND_GROUP_LIST: u32 = 9101;

/// Name of the blend group | Group name
pub const AC_BLENDGROUP_NAME: u32 = 9102;

/// Column Name | Human readable column name (debugging)
pub const G2DA_COLUMN_NAME: u32 = 10000;

/// Column Hash | 32-bit hash ID of column name string
pub const G2DA_COLUMN_HASH: u32 = 10001;

/// Column List | Table of column information
pub const G2DA_COLUMN_LIST: u32 = 10002;

/// Row List | Table of row information
pub const G2DA_ROW_LIST: u32 = 10003;

/// Row Data | Structure containing row variable data
pub const G2DA_ROW_DATA: u32 = 10004;

/// Column Index
pub const G2DA_COLUMN_1: u32 = 10005;

/// Column Index
pub const G2DA_COLUMN_2: u32 = 10006;

/// Column Index
pub const G2DA_COLUMN_3: u32 = 10007;

/// Column Index
pub const G2DA_COLUMN_4: u32 = 10008;

/// Column Index
pub const G2DA_COLUMN_5: u32 = 10009;

/// Column Index
pub const G2DA_COLUMN_6: u32 = 10010;

/// Column Index
pub const G2DA_COLUMN_7: u32 = 10011;

/// Column Index
pub const G2DA_COLUMN_8: u32 = 10012;

/// Column Index
pub const G2DA_COLUMN_9: u32 = 10013;

/// Column Index
pub const G2DA_COLUMN_10: u32 = 10014;

/// Column Index
pub const G2DA_COLUMN_11: u32 = 10015;

/// Column Index
pub const G2DA_COLUMN_12: u32 = 10016;

/// Column Index
pub const G2DA_COLUMN_13: u32 = 10017;

/// Column Index
pub const G2DA_COLUMN_14: u32 = 10018;

/// Column Index
pub const G2DA_COLUMN_15: u32 = 10019;

/// Column Index
pub const G2DA_COLUMN_16: u32 = 10020;

/// Column Index
pub const G2DA_COLUMN_17: u32 = 10021;

/// Column Index
pub const G2DA_COLUMN_18: u32 = 10022;

/// Column Index
pub const G2DA_COLUMN_19: u32 = 10023;

/// Column Index
pub const G2DA_COLUMN_20: u32 = 10024;

/// Column Index
pub const G2DA_COLUMN_21: u32 = 10025;

/// Column Index
pub const G2DA_COLUMN_22: u32 = 10026;

/// Column Index
pub const G2DA_COLUMN_23: u32 = 10027;

/// Column Index
pub const G2DA_COLUMN_24: u32 = 10028;

/// Column Index
pub const G2DA_COLUMN_25: u32 = 10029;

/// Column Index
pub const G2DA_COLUMN_26: u32 = 10030;

/// Column Index
pub const G2DA_COLUMN_27: u32 = 10031;

/// Column Index
pub const G2DA_COLUMN_28: u32 = 10032;

/// Column Index
pub const G2DA_COLUMN_29: u32 = 10033;

/// Column Index
pub const G2DA_COLUMN_30: u32 = 10034;

/// Column Index
pub const G2DA_COLUMN_31: u32 = 10035;

/// Column Index
pub const G2DA_COLUMN_32: u32 = 10036;

/// Column Index
pub const G2DA_COLUMN_33: u32 = 10037;

/// Column Index
pub const G2DA_COLUMN_34: u32 = 10038;

/// Column Index
pub const G2DA_COLUMN_35: u32 = 10039;

/// Column Index
pub const G2DA_COLUMN_36: u32 = 10040;

/// Column Index
pub const G2DA_COLUMN_37: u32 = 10041;

/// Column Index
pub const G2DA_COLUMN_38: u32 = 10042;

/// Column Index
pub const G2DA_COLUMN_39: u32 = 10043;

/// Column Index
pub const G2DA_COLUMN_40: u32 = 10044;

/// Column Index
pub const G2DA_COLUMN_41: u32 = 10045;

/// Column Index
pub const G2DA_COLUMN_42: u32 = 10046;

/// Column Index
pub const G2DA_COLUMN_43: u32 = 10047;

/// Column Index
pub const G2DA_COLUMN_44: u32 = 10048;

/// Column Index
pub const G2DA_COLUMN_45: u32 = 10049;

/// Column Index
pub const G2DA_COLUMN_46: u32 = 10050;

/// Column Index
pub const G2DA_COLUMN_47: u32 = 10051;

/// Column Index
pub const G2DA_COLUMN_48: u32 = 10052;

/// Column Index
pub const G2DA_COLUMN_49: u32 = 10053;

/// Column Index
pub const G2DA_COLUMN_50: u32 = 10054;

/// Column Index
pub const G2DA_COLUMN_51: u32 = 10055;

/// Column Index
pub const G2DA_COLUMN_52: u32 = 10056;

/// Column Index
pub const G2DA_COLUMN_53: u32 = 10057;

/// Column Index
pub const G2DA_COLUMN_54: u32 = 10058;

/// Column Index
pub const G2DA_COLUMN_55: u32 = 10059;

/// Column Index
pub const G2DA_COLUMN_56: u32 = 10060;

/// Column Index
pub const G2DA_COLUMN_57: u32 = 10061;

/// Column Index
pub const G2DA_COLUMN_58: u32 = 10062;

/// Column Index
pub const G2DA_COLUMN_59: u32 = 10063;

/// Column Index
pub const G2DA_COLUMN_60: u32 = 10064;

/// Column Index
pub const G2DA_COLUMN_61: u32 = 10065;

/// Column Index
pub const G2DA_COLUMN_62: u32 = 10066;

/// Column Index
pub const G2DA_COLUMN_63: u32 = 10067;

/// Column Index
pub const G2DA_COLUMN_64: u32 = 10068;

/// Column Index
pub const G2DA_COLUMN_65: u32 = 10069;

/// Column Index
pub const G2DA_COLUMN_66: u32 = 10070;

/// Column Index
pub const G2DA_COLUMN_67: u32 = 10071;

/// Column Index
pub const G2DA_COLUMN_68: u32 = 10072;

/// Column Index
pub const G2DA_COLUMN_69: u32 = 10073;

/// Column Index
pub const G2DA_COLUMN_70: u32 = 10074;

/// Column Index
pub const G2DA_COLUMN_71: u32 = 10075;

/// Column Index
pub const G2DA_COLUMN_72: u32 = 10076;

/// Column Index
pub const G2DA_COLUMN_73: u32 = 10077;

/// Column Index
pub const G2DA_COLUMN_74: u32 = 10078;

/// Column Index
pub const G2DA_COLUMN_75: u32 = 10079;

/// Column Index
pub const G2DA_COLUMN_76: u32 = 10080;

/// Column Index
pub const G2DA_COLUMN_77: u32 = 10081;

/// Column Index
pub const G2DA_COLUMN_78: u32 = 10082;

/// Column Index
pub const G2DA_COLUMN_79: u32 = 10083;

/// Column Index
pub const G2DA_COLUMN_80: u32 = 10084;

/// Column Index
pub const G2DA_COLUMN_81: u32 = 10085;

/// Column Index
pub const G2DA_COLUMN_82: u32 = 10086;

/// Column Index
pub const G2DA_COLUMN_83: u32 = 10087;

/// Column Index
pub const G2DA_COLUMN_84: u32 = 10088;

/// Column Index
pub const G2DA_COLUMN_85: u32 = 10089;

/// Column Index
pub const G2DA_COLUMN_86: u32 = 10090;

/// Column Index
pub const G2DA_COLUMN_87: u32 = 10091;

/// Column Index
pub const G2DA_COLUMN_88: u32 = 10092;

/// Column Index
pub const G2DA_COLUMN_89: u32 = 10093;

/// Column Index
pub const G2DA_COLUMN_90: u32 = 10094;

/// Column Index
pub const G2DA_COLUMN_91: u32 = 10095;

/// Column Index
pub const G2DA_COLUMN_92: u32 = 10096;

/// Column Index
pub const G2DA_COLUMN_93: u32 = 10097;

/// Column Index
pub const G2DA_COLUMN_94: u32 = 10098;

/// Column Index
pub const G2DA_COLUMN_95: u32 = 10099;

/// Column Index
pub const G2DA_COLUMN_96: u32 = 10100;

/// Column Index
pub const G2DA_COLUMN_97: u32 = 10101;

/// Column Index
pub const G2DA_COLUMN_98: u32 = 10102;

/// Column Index
pub const G2DA_COLUMN_99: u32 = 10103;

/// Column Index
pub const G2DA_COLUMN_100: u32 = 10104;

/// Column Index
pub const G2DA_COLUMN_101: u32 = 10105;

/// Column Index
pub const G2DA_COLUMN_102: u32 = 10106;

/// Column Index
pub const G2DA_COLUMN_103: u32 = 10107;

/// Column Index
pub const G2DA_COLUMN_104: u32 = 10108;

/// Column Index
pub const G2DA_COLUMN_105: u32 = 10109;

/// Column Index
pub const G2DA_COLUMN_106: u32 = 10110;

/// Column Index
pub const G2DA_COLUMN_107: u32 = 10111;

/// Column Index
pub const G2DA_COLUMN_108: u32 = 10112;

/// Column Index
pub const G2DA_COLUMN_109: u32 = 10113;

/// Column Index
pub const G2DA_COLUMN_110: u32 = 10114;

/// Column Index
pub const G2DA_COLUMN_111: u32 = 10115;

/// Column Index
pub const G2DA_COLUMN_112: u32 = 10116;

/// Column Index
pub const G2DA_COLUMN_113: u32 = 10117;

/// Column Index
pub const G2DA_COLUMN_114: u32 = 10118;

/// Column Index
pub const G2DA_COLUMN_115: u32 = 10119;

/// Column Index
pub const G2DA_COLUMN_116: u32 = 10120;

/// Column Index
pub const G2DA_COLUMN_117: u32 = 10121;

/// Column Index
pub const G2DA_COLUMN_118: u32 = 10122;

/// Column Index
pub const G2DA_COLUMN_119: u32 = 10123;

/// Column Index
pub const G2DA_COLUMN_120: u32 = 10124;

/// Column Index
pub const G2DA_COLUMN_121: u32 = 10125;

/// Column Index
pub const G2DA_COLUMN_122: u32 = 10126;

/// Column Index
pub const G2DA_COLUMN_123: u32 = 10127;

/// Column Index
pub const G2DA_COLUMN_124: u32 = 10128;

/// Column Index
pub const G2DA_COLUMN_125: u32 = 10129;

/// Column Index
pub const G2DA_COLUMN_126: u32 = 10130;

/// Column Index
pub const G2DA_COLUMN_127: u32 = 10131;

/// Column Index
pub const G2DA_COLUMN_128: u32 = 10132;

/// Column Index
pub const G2DA_COLUMN_129: u32 = 10133;

/// Column Index
pub const G2DA_COLUMN_130: u32 = 10134;

/// Column Index
pub const G2DA_COLUMN_131: u32 = 10135;

/// Column Index
pub const G2DA_COLUMN_132: u32 = 10136;

/// Column Index
pub const G2DA_COLUMN_133: u32 = 10137;

/// Column Index
pub const G2DA_COLUMN_134: u32 = 10138;

/// Column Index
pub const G2DA_COLUMN_135: u32 = 10139;

/// Column Index
pub const G2DA_COLUMN_136: u32 = 10140;

/// Column Index
pub const G2DA_COLUMN_137: u32 = 10141;

/// Column Index
pub const G2DA_COLUMN_138: u32 = 10142;

/// Column Index
pub const G2DA_COLUMN_139: u32 = 10143;

/// Column Index
pub const G2DA_COLUMN_140: u32 = 10144;

/// Column Index
pub const G2DA_COLUMN_141: u32 = 10145;

/// Column Index
pub const G2DA_COLUMN_142: u32 = 10146;

/// Column Index
pub const G2DA_COLUMN_143: u32 = 10147;

/// Column Index
pub const G2DA_COLUMN_144: u32 = 10148;

/// Column Index
pub const G2DA_COLUMN_145: u32 = 10149;

/// Column Index
pub const G2DA_COLUMN_146: u32 = 10150;

/// Column Index
pub const G2DA_COLUMN_147: u32 = 10151;

/// Column Index
pub const G2DA_COLUMN_148: u32 = 10152;

/// Column Index
pub const G2DA_COLUMN_149: u32 = 10153;

/// Column Index
pub const G2DA_COLUMN_150: u32 = 10154;

/// Column Index
pub const G2DA_COLUMN_151: u32 = 10155;

/// Column Index
pub const G2DA_COLUMN_152: u32 = 10156;

/// Column Index
pub const G2DA_COLUMN_153: u32 = 10157;

/// Column Index
pub const G2DA_COLUMN_154: u32 = 10158;

/// Column Index
pub const G2DA_COLUMN_155: u32 = 10159;

/// Column Index
pub const G2DA_COLUMN_156: u32 = 10160;

/// Column Index
pub const G2DA_COLUMN_157: u32 = 10161;

/// Column Index
pub const G2DA_COLUMN_158: u32 = 10162;

/// Column Index
pub const G2DA_COLUMN_159: u32 = 10163;

/// Column Index
pub const G2DA_COLUMN_160: u32 = 10164;

/// Column Index
pub const G2DA_COLUMN_161: u32 = 10165;

/// Column Index
pub const G2DA_COLUMN_162: u32 = 10166;

/// Column Index
pub const G2DA_COLUMN_163: u32 = 10167;

/// Column Index
pub const G2DA_COLUMN_164: u32 = 10168;

/// Column Index
pub const G2DA_COLUMN_165: u32 = 10169;

/// Column Index
pub const G2DA_COLUMN_166: u32 = 10170;

/// Column Index
pub const G2DA_COLUMN_167: u32 = 10171;

/// Column Index
pub const G2DA_COLUMN_168: u32 = 10172;

/// Column Index
pub const G2DA_COLUMN_169: u32 = 10173;

/// Column Index
pub const G2DA_COLUMN_170: u32 = 10174;

/// Column Index
pub const G2DA_COLUMN_171: u32 = 10175;

/// Column Index
pub const G2DA_COLUMN_172: u32 = 10176;

/// Column Index
pub const G2DA_COLUMN_173: u32 = 10177;

/// Column Index
pub const G2DA_COLUMN_174: u32 = 10178;

/// Column Index
pub const G2DA_COLUMN_175: u32 = 10179;

/// Column Index
pub const G2DA_COLUMN_176: u32 = 10180;

/// Column Index
pub const G2DA_COLUMN_177: u32 = 10181;

/// Column Index
pub const G2DA_COLUMN_178: u32 = 10182;

/// Column Index
pub const G2DA_COLUMN_179: u32 = 10183;

/// Column Index
pub const G2DA_COLUMN_180: u32 = 10184;

/// Column Index
pub const G2DA_COLUMN_181: u32 = 10185;

/// Column Index
pub const G2DA_COLUMN_182: u32 = 10186;

/// Column Index
pub const G2DA_COLUMN_183: u32 = 10187;

/// Column Index
pub const G2DA_COLUMN_184: u32 = 10188;

/// Column Index
pub const G2DA_COLUMN_185: u32 = 10189;

/// Column Index
pub const G2DA_COLUMN_186: u32 = 10190;

/// Column Index
pub const G2DA_COLUMN_187: u32 = 10191;

/// Column Index
pub const G2DA_COLUMN_188: u32 = 10192;

/// Column Index
pub const G2DA_COLUMN_189: u32 = 10193;

/// Column Index
pub const G2DA_COLUMN_190: u32 = 10194;

/// Column Index
pub const G2DA_COLUMN_191: u32 = 10195;

/// Column Index
pub const G2DA_COLUMN_192: u32 = 10196;

/// Column Index
pub const G2DA_COLUMN_193: u32 = 10197;

/// Column Index
pub const G2DA_COLUMN_194: u32 = 10198;

/// Column Index
pub const G2DA_COLUMN_195: u32 = 10199;

/// Column Index
pub const G2DA_COLUMN_196: u32 = 10200;

/// Column Index
pub const G2DA_COLUMN_197: u32 = 10201;

/// Column Index
pub const G2DA_COLUMN_198: u32 = 10202;

/// Column Index
pub const G2DA_COLUMN_199: u32 = 10203;

/// Column Index
pub const G2DA_COLUMN_200: u32 = 10204;

/// Column Index
pub const G2DA_COLUMN_201: u32 = 10205;

/// Column Index
pub const G2DA_COLUMN_202: u32 = 10206;

/// Column Index
pub const G2DA_COLUMN_203: u32 = 10207;

/// Column Index
pub const G2DA_COLUMN_204: u32 = 10208;

/// Column Index
pub const G2DA_COLUMN_205: u32 = 10209;

/// Column Index
pub const G2DA_COLUMN_206: u32 = 10210;

/// Column Index
pub const G2DA_COLUMN_207: u32 = 10211;

/// Column Index
pub const G2DA_COLUMN_208: u32 = 10212;

/// Column Index
pub const G2DA_COLUMN_209: u32 = 10213;

/// Column Index
pub const G2DA_COLUMN_210: u32 = 10214;

/// Column Index
pub const G2DA_COLUMN_211: u32 = 10215;

/// Column Index
pub const G2DA_COLUMN_212: u32 = 10216;

/// Column Index
pub const G2DA_COLUMN_213: u32 = 10217;

/// Column Index
pub const G2DA_COLUMN_214: u32 = 10218;

/// Column Index
pub const G2DA_COLUMN_215: u32 = 10219;

/// Column Index
pub const G2DA_COLUMN_216: u32 = 10220;

/// Column Index
pub const G2DA_COLUMN_217: u32 = 10221;

/// Column Index
pub const G2DA_COLUMN_218: u32 = 10222;

/// Column Index
pub const G2DA_COLUMN_219: u32 = 10223;

/// Column Index
pub const G2DA_COLUMN_220: u32 = 10224;

/// Column Index
pub const G2DA_COLUMN_221: u32 = 10225;

/// Column Index
pub const G2DA_COLUMN_222: u32 = 10226;

/// Column Index
pub const G2DA_COLUMN_223: u32 = 10227;

/// Column Index
pub const G2DA_COLUMN_224: u32 = 10228;

/// Column Index
pub const G2DA_COLUMN_225: u32 = 10229;

/// Column Index
pub const G2DA_COLUMN_226: u32 = 10230;

/// Column Index
pub const G2DA_COLUMN_227: u32 = 10231;

/// Column Index
pub const G2DA_COLUMN_228: u32 = 10232;

/// Column Index
pub const G2DA_COLUMN_229: u32 = 10233;

/// Column Index
pub const G2DA_COLUMN_230: u32 = 10234;

/// Column Index
pub const G2DA_COLUMN_231: u32 = 10235;

/// Column Index
pub const G2DA_COLUMN_232: u32 = 10236;

/// Column Index
pub const G2DA_COLUMN_233: u32 = 10237;

/// Column Index
pub const G2DA_COLUMN_234: u32 = 10238;

/// Column Index
pub const G2DA_COLUMN_235: u32 = 10239;

/// Column Index
pub const G2DA_COLUMN_236: u32 = 10240;

/// Column Index
pub const G2DA_COLUMN_237: u32 = 10241;

/// Column Index
pub const G2DA_COLUMN_238: u32 = 10242;

/// Column Index
pub const G2DA_COLUMN_239: u32 = 10243;

/// Column Index
pub const G2DA_COLUMN_240: u32 = 10244;

/// Column Index
pub const G2DA_COLUMN_241: u32 = 10245;

/// Column Index
pub const G2DA_COLUMN_242: u32 = 10246;

/// Column Index
pub const G2DA_COLUMN_243: u32 = 10247;

/// Column Index
pub const G2DA_COLUMN_244: u32 = 10248;

/// Column Index
pub const G2DA_COLUMN_245: u32 = 10249;

/// Column Index
pub const G2DA_COLUMN_246: u32 = 10250;

/// Column Index
pub const G2DA_COLUMN_247: u32 = 10251;

/// Column Index
pub const G2DA_COLUMN_248: u32 = 10252;

/// Column Index
pub const G2DA_COLUMN_249: u32 = 10253;

/// Column Index
pub const G2DA_COLUMN_250: u32 = 10254;

/// Column Index
pub const G2DA_COLUMN_251: u32 = 10255;

/// Column Index
pub const G2DA_COLUMN_252: u32 = 10256;

/// Column Index
pub const G2DA_COLUMN_253: u32 = 10257;

/// Column Index
pub const G2DA_COLUMN_254: u32 = 10258;

/// Column Index
pub const G2DA_COLUMN_255: u32 = 10259;

/// Column Index
pub const G2DA_COLUMN_256: u32 = 10260;

/// Column Index
pub const G2DA_COLUMN_257: u32 = 10261;

/// Column Index
pub const G2DA_COLUMN_258: u32 = 10262;

/// Column Index
pub const G2DA_COLUMN_259: u32 = 10263;

/// Column Index
pub const G2DA_COLUMN_260: u32 = 10264;

/// Column Index
pub const G2DA_COLUMN_261: u32 = 10265;

/// Column Index
pub const G2DA_COLUMN_262: u32 = 10266;

/// Column Index
pub const G2DA_COLUMN_263: u32 = 10267;

/// Column Index
pub const G2DA_COLUMN_264: u32 = 10268;

/// Column Index
pub const G2DA_COLUMN_265: u32 = 10269;

/// Column Index
pub const G2DA_COLUMN_266: u32 = 10270;

/// Column Index
pub const G2DA_COLUMN_267: u32 = 10271;

/// Column Index
pub const G2DA_COLUMN_268: u32 = 10272;

/// Column Index
pub const G2DA_COLUMN_269: u32 = 10273;

/// Column Index
pub const G2DA_COLUMN_270: u32 = 10274;

/// Column Index
pub const G2DA_COLUMN_271: u32 = 10275;

/// Column Index
pub const G2DA_COLUMN_272: u32 = 10276;

/// Column Index
pub const G2DA_COLUMN_273: u32 = 10277;

/// Column Index
pub const G2DA_COLUMN_274: u32 = 10278;

/// Column Index
pub const G2DA_COLUMN_275: u32 = 10279;

/// Column Index
pub const G2DA_COLUMN_276: u32 = 10280;

/// Column Index
pub const G2DA_COLUMN_277: u32 = 10281;

/// Column Index
pub const G2DA_COLUMN_278: u32 = 10282;

/// Column Index
pub const G2DA_COLUMN_279: u32 = 10283;

/// Column Index
pub const G2DA_COLUMN_280: u32 = 10284;

/// Column Index
pub const G2DA_COLUMN_281: u32 = 10285;

/// Column Index
pub const G2DA_COLUMN_282: u32 = 10286;

/// Column Index
pub const G2DA_COLUMN_283: u32 = 10287;

/// Column Index
pub const G2DA_COLUMN_284: u32 = 10288;

/// Column Index
pub const G2DA_COLUMN_285: u32 = 10289;

/// Column Index
pub const G2DA_COLUMN_286: u32 = 10290;

/// Column Index
pub const G2DA_COLUMN_287: u32 = 10291;

/// Column Index
pub const G2DA_COLUMN_288: u32 = 10292;

/// Column Index
pub const G2DA_COLUMN_289: u32 = 10293;

/// Column Index
pub const G2DA_COLUMN_290: u32 = 10294;

/// Column Index
pub const G2DA_COLUMN_291: u32 = 10295;

/// Column Index
pub const G2DA_COLUMN_292: u32 = 10296;

/// Column Index
pub const G2DA_COLUMN_293: u32 = 10297;

/// Column Index
pub const G2DA_COLUMN_294: u32 = 10298;

/// Column Index
pub const G2DA_COLUMN_295: u32 = 10299;

/// Column Index
pub const G2DA_COLUMN_296: u32 = 10300;

/// Column Index
pub const G2DA_COLUMN_297: u32 = 10301;

/// Column Index
pub const G2DA_COLUMN_298: u32 = 10302;

/// Column Index
pub const G2DA_COLUMN_299: u32 = 10303;

/// Column Type | enum of column data type
pub const G2DA_COLUMN_TYPE: u32 = 10999;

/// Place List | List of places
pub const STAGE_PLACE_LIST: u32 = 11000;

/// Camera List | List of cameras
pub const STAGE_CAMERA_LIST: u32 = 11001;

/// Places in Shot | List of places viewed by the camera
pub const STAGE_PLACES_IN_SHOT: u32 = 11002;

/// FOV | Camera field of view
pub const STAGE_CAMERA_FOV: u32 = 11003;

/// Default Camera | The default camera for the place
pub const STAGE_PLACE_DEFAULT_CAMERA: u32 = 11004;

/// Deprecated | Deprecated
pub const STAGE_CAMERA_DEPRECATED: u32 = 11005;

/// Look At Type | The type of lookat to be used
pub const STAGE_CAMERA_LOOKING_AT_TYPE: u32 = 11009;

/// Starting List | List of starts
pub const CONVERSATION_STARTING_LIST: u32 = 12000;

/// Starting Index | Index of the starting node
pub const CONVERSATION_STARTING_INDEX: u32 = 12001;

/// Line List | List of conversation lines
pub const CONVERSATION_LINE_LIST: u32 = 12002;

/// End Action | Actions executed when conversation ends
pub const CONVERSATION_END: u32 = 12003;

/// VO Bank | VO Soundbank
pub const CONVERSATION_VOBANK: u32 = 12004;

/// Stage Name | Name of the stage
pub const CONVERSATION_STAGE_NAME: u32 = 12100;

/// Stage Map | Mapping of object tags to place tags
pub const CONVERSATION_STAGE_MAP: u32 = 12101;

/// Key Tag | Key tag of the map
pub const CONVERSATION_KEY_TAG: u32 = 12102;

/// Value Tag | Value tag of the map
pub const CONVERSATION_VALUE_TAG: u32 = 12103;

/// / Stage At Current Location | Whether to place the stage origin at the location of the conversation owner
pub const CONVERSATION_STAGE_AT_CURRENT_LOCATION: u32 = 12104;

/// Text | Text of the line
pub const CONVERSATION_LINE_TEXT: u32 = 12201;

/// Speaker | The speaker of the line
pub const CONVERSATION_LINE_SPEAKER: u32 = 12202;

/// Listener | The listener of the line
pub const CONVERSATION_LINE_LISTENER: u32 = 12203;

/// Icon | The line's icon
pub const CONVERSATION_LINE_ICON: u32 = 12205;

/// Visibility | The visibility of the line
pub const CONVERSATION_LINE_VISIBILITY: u32 = 12206;

/// Ambient | Whether the line is ambient
pub const CONVERSATION_LINE_AMBIENT: u32 = 12207;

/// Condition | Conditional parameters of the line
pub const CONVERSATION_LINE_COND: u32 = 12208;

/// Action | Actions to take on the line
pub const CONVERSATION_LINE_ACTION: u32 = 12209;

/// Cutscene | Embedded cutscene
pub const CONVERSATION_LINE_CUTSCENE: u32 = 12211;

/// Cutscene Map | Tag map for cutscene
pub const CONVERSATION_LINE_CUTSCENE_MAP: u32 = 12212;

/// Speaker animation | Speaker animation
pub const CONVERSATION_LINE_ANIMATION: u32 = 12213;

/// Skip line | If true, don't play dialog on this line (cinematic conversations)
pub const CONVERSATION_LINE_SKIP: u32 = 12214;

/// Fast path | Quick choice for players who want to pick certain path (eg. fast/neutral)
pub const CONVERSATION_LINE_FASTPATH: u32 = 12215;

/// Revert anim | If true, revert speaker animation to idle at the end of the line
pub const CONVERSATION_LINE_REVERT_ANIM: u32 = 12217;

/// Plot GUID | The GUID of the plot to use
pub const CONVERSATION_PLOT_GUID: u32 = 12300;

/// Plot Flag | The flag on the plot to use
pub const CONVERSATION_PLOT_FLAG: u32 = 12301;

/// Plot Test | The test case to use on the plot
pub const CONVERSATION_PLOT_TEST: u32 = 12302;

/// Script | The script to use
pub const CONVERSATION_SCRIPT: u32 = 12303;

/// Script Parameter | The parameter to the script
pub const CONVERSATION_SCRIPT_PARAMETER: u32 = 12304;

/// Children | List of children of this line
pub const CONVERSATION_LINE_CHILDREN_LIST: u32 = 12400;

/// Flags | List of plot flags
pub const PLOT_FLAGS: u32 = 13000;

/// ID | ID to refer to the flag by
pub const PLOT_FLAG_ID: u32 = 13001;

/// Name | Name of the plot flag
pub const PLOT_FLAG_NAME: u32 = 13002;

/// Reward | ID of the plot reward
pub const PLOT_FLAG_REWARD: u32 = 13003;

/// Journal | Journal text for the flag
pub const PLOT_FLAG_JOURNAL: u32 = 13004;

/// Ends Plot | Whether this flag ends the plot
pub const PLOT_FLAG_ENDS_PLOT: u32 = 13005;

/// GUID | GUID of the plot
pub const PLOT_GUID: u32 = 13007;

/// Name | Name of the plot
pub const PLOT_NAME: u32 = 13008;

/// Script | The script file associated with the plot
pub const PLOT_SCRIPT: u32 = 13009;

/// Priority | The priority of the plot
pub const PLOT_PRIORITY: u32 = 13010;

/// Flags1 | Default values of the plot flags
pub const PLOT_FLAGS1: u32 = 13011;

/// Flags2 | Default values of the plot flags
pub const PLOT_FLAGS2: u32 = 13012;

/// Flags3 | Default values of the plot flags
pub const PLOT_FLAGS3: u32 = 13013;

/// Flags4 | Default values of the plot flags
pub const PLOT_FLAGS4: u32 = 13014;

/// Plots | List of plots
pub const PLOT_PLOTS: u32 = 13016;

/// Parent Plot | Plot that this is a sub-plot of
pub const PLOT_PARENT_PLOT: u32 = 13017;

/// AssistInfo | list of plot assist info
pub const PLOTASSIST_LIST: u32 = 13019;

/// PlotAdvancerTag | tag of something that becomes or stops being a plot destination
pub const PLOTASSIST_TAG: u32 = 13020;

/// Entry Type | Type of plot entry (plot vs. codex)
pub const PLOT_ENTRYTYPE: u32 = 13022;

/// Allow Pausing | true if plot allows pausing.
pub const PLOT_ALLOW_PAUSING: u32 = 13023;

/// OfferID | Offer this plot flag is associated with
pub const PLOT_FLAG_OFFERID: u32 = 13024;

/// Parent Plot GUID | Plot GUID that this is a sub-plot of
pub const PLOT_PARENT_PLOT_GUID: u32 = 13025;

/// Diff Red Channel | Diffuse Tint mask Red channel
pub const TINT_MASK_DIFFUSE_R: u32 = 14000;

/// Diff Green Channel | Diffuse Tint mask Green channel
pub const TINT_MASK_DIFFUSE_G: u32 = 14001;

/// Diff Blue Channel | Diffuse Tint mask Blue channel
pub const TINT_MASK_DIFFUSE_B: u32 = 14002;

/// Spec Red Channel | Specular Tint mask Red channel
pub const TINT_MASK_SPECULAR_R: u32 = 14003;

/// Spec Green Channel | Specular Tint mask Green channel
pub const TINT_MASK_SPECULAR_G: u32 = 14004;

/// Spec Blue Channel | Specular Tint mask Blue channel
pub const TINT_MASK_SPECULAR_B: u32 = 14005;

/// Diffuse Alpha Channel | Diffuse tint mask alpha channel
pub const TINT_MASK_DIFFUSE_A: u32 = 14006;

/// Material project file version
pub const MAT_FILE_OBJECT_VERSION: u32 = 15000;

/// Child list | material editor child list
pub const MAT_CHILD_LIST: u32 = 15001;

/// Root | Root object in material editor
pub const MAT_ROOT: u32 = 15010;

/// Root name | Name of the root pbject
pub const MAT_ROOT_NAME: u32 = 15011;

/// Model | Model object in material editor
pub const MAT_MODEL: u32 = 15012;

/// Model name | Name of the model
pub const MAT_MODEL_NAME: u32 = 15013;

/// Model part | Part object in the model
pub const MAT_PART: u32 = 15014;

/// Part name | Name of the part object
pub const MAT_PART_NAME: u32 = 15015;

/// Part MMH parent | Parent of the part in MMH hierarchy
pub const MAT_PART_MMH_PARENT: u32 = 15016;

/// MatLib | Material library object in material editor
pub const MAT_MATLIB: u32 = 15017;

/// MatLib name | Name of the material library
pub const MAT_MATLIB_NAME: u32 = 15018;

/// MatObj | Material object in the material editor
pub const MAT_MATOBJ: u32 = 15019;

/// MatObj name | Name of the material object
pub const MAT_MATOBJ_NAME: u32 = 15020;

/// Light | Light object in material editor
pub const MAT_LIGHT: u32 = 15021;

/// Light name | Name of the light object
pub const MAT_LIGHT_NAME: u32 = 15022;

/// Light rig | Light rig object in material editor
pub const MAT_LIGHT_RIG: u32 = 15023;

/// Light rig name | Name of the light rig object
pub const MAT_LIGHT_RIG_NAME: u32 = 15024;

/// Light probe | Light probe object in material editor
pub const MAT_LIGHT_PROBE: u32 = 15025;

/// Light probe name | Name of light probe
pub const MAT_LIGHT_PROBE_NAME: u32 = 15026;

/// Group | Group object in material editor
pub const MAT_GROUP: u32 = 15027;

/// Group name | Name of the group object
pub const MAT_GROUP_NAME: u32 = 15028;

/// PaletteLib | Palette library object in material editor
pub const MAT_PALETTELIB: u32 = 15029;

/// PaletteLib name | Name of the palette library
pub const MAT_PALETTELIB_NAME: u32 = 15030;

/// PaletteObj | Palette object in the material editor
pub const MAT_PALETTEOBJ: u32 = 15031;

/// PaletteObj name | Name of the palette object
pub const MAT_PALETTEOBJ_NAME: u32 = 15032;

/// HeraldryLib | Heraldry library object in material editor
pub const MAT_HERALDRYLIB: u32 = 15033;

/// HeraldryLib name | Name of the Heraldry library
pub const MAT_HERALDRYLIB_NAME: u32 = 15034;

/// HeraldryObj | Heraldry object in the material editor
pub const MAT_HERALDRYOBJ: u32 = 15035;

/// HeraldryObj name | Name of the Heraldry object
pub const MAT_HERALDRYOBJ_NAME: u32 = 15036;

/// Duplicate | Duplicate object in material editor
pub const MAT_DUPLICATE: u32 = 15037;

/// Duplicate name | Name of the Duplicate
pub const MAT_DUPLICATE_NAME: u32 = 15038;

/// Layout name | Name of the layout
pub const MAT_LAYOUT_NAME: u32 = 15039;

/// TintLib | Tint library object in material editor
pub const MAT_TINTLIB: u32 = 15040;

/// TintLib name | Name of the tint library
pub const MAT_TINTLIB_NAME: u32 = 15041;

/// TintObj | Tint object in the material editor
pub const MAT_TINTOBJ: u32 = 15042;

/// TintObj name | Name of the tint object
pub const MAT_TINTOBJ_NAME: u32 = 15043;

/// Material type | Material type, static or character
pub const MAT_MATERIAL_TYPE: u32 = 15050;

/// Basic parameters | Basic material parameters
pub const MAT_BASIC_PARAMS: u32 = 15051;

/// Shiny Transparent | Shiny transparent
pub const MAT_SHINY_TRANS: u32 = 15052;

/// Two side | Two sided material
pub const MAT_TWO_SIDE: u32 = 15053;

/// Hair | Hair material
pub const MAT_HAIR: u32 = 15054;

/// Dynamic light | Dynamic light mode
pub const MAT_DYNC_LIGHT: u32 = 15055;

/// Blend mode | Blend mode for the material
pub const MAT_BLEND_MODE: u32 = 15056;

/// Mat name | Name of the material object used by the MMH file
pub const MAT_NAME: u32 = 15057;

/// Material type | Material type, string
pub const MAT_MATERIAL_TYPE_STRING: u32 = 15058;

/// Material Semantic | Material Semantic
pub const MAT_MATERIAL_SEMANTIC: u32 = 15059;

/// Material sound type | Material sound type
pub const MAT_MATERIAL_SOUND_TYPE: u32 = 15060;

/// Diffuse map type | Type of the diffuse map
pub const MAT_DIFFUSE_MAP_TYPE: u32 = 15070;

/// Diffuse map color | Color of the diffuse map
pub const MAT_DIFFUSE_MAP_COLOR: u32 = 15071;

/// Diffuse map scale | Scale of the diffuse map
pub const MAT_DIFFUSE_MAP_SCALE: u32 = 15072;

/// Diffuse map | Texture of the diffuse map
pub const MAT_DIFFUSE_MAP: u32 = 15073;

/// Diffuse file name | Diffuse map name used in MAL file
pub const MAT_DIFFUSE_FILENAME: u32 = 15074;

/// Diffuse/opacity dimension X | Dimension X of the diffuse map used in MAL file
pub const MAT_DIFFOPAC_DIMENSIONX: u32 = 15075;

/// Diffuse/opacity dimension Y | Dimension Y of the diffuse map used in MAL file
pub const MAT_DIFFOPAC_DIMENSIONY: u32 = 15076;

/// Diffuse/opacity compression | Compression type
pub const MAT_DIFFOPAC_COMPRESSION: u32 = 15077;

/// Opacity map enable | Enable/disable using opacity map
pub const MAT_OPACITYMAPENABLE: u32 = 15100;

/// Opacity map type | Type of the opacity map
pub const MAT_OPACITYMAPTYPE: u32 = 15101;

/// Opacity map color | Color of the opacity map
pub const MAT_OPACITYMAPCOLOR: u32 = 15102;

/// Opacity map scale | Scale of the opacity map
pub const MAT_OPACITYMAPSCALE: u32 = 15103;

/// Opacity map | Texture file of the opacity map
pub const MAT_OPACITYMAP: u32 = 15104;

/// Specular map type | Type of the specular map
pub const MAT_SPECULAR_MAP_TYPE: u32 = 15131;

/// Specular map color | Color of the specular map
pub const MAT_SPECULAR_MAP_COLOR: u32 = 15132;

/// Specular map scale | Scale of the specular map
pub const MAT_SPECULAR_MAP_SCALE: u32 = 15133;

/// Specualr map | Texture file of the specualr map
pub const MAT_SPECULAR_MAP: u32 = 15134;

/// Specular file name | Texture file used in MAL file
pub const MAT_SPECULAR_FILENAME: u32 = 15139;

/// Normal map enable | Enable/disable normal map
pub const MAT_NORMAL_MAP_ENABLE: u32 = 15160;

/// Normal map | Texture of the normal map
pub const MAT_NORMAL_MAP: u32 = 15161;

/// Normal map file name | Texture file used in MAL file
pub const MAT_NORMAL_FILENAME: u32 = 15162;

/// Normal map compression | Compression Type
pub const MAT_NORMAL_COMPRESSION: u32 = 15163;

/// Tint map enable | Enable/disable tint map
pub const MAT_TINT_MAP_ENABLE: u32 = 15190;

/// Tint map | Texture of the tine map
pub const MAT_TINT_MAP: u32 = 15191;

/// Tint R enable | Enable/disable R channel
pub const MAT_TINT_R_ENABLE: u32 = 15192;

/// Tint G enable | Enable/disable G channel
pub const MAT_TINT_G_ENABLE: u32 = 15193;

/// Tint B enable | Enable/disable B channel
pub const MAT_TINT_B_ENABLE: u32 = 15194;

/// TNT file name | Name of the TNT file
pub const MAT_TINT_FILENAME_POSTFIX: u32 = 15195;

/// Tint map compression | Compression type
pub const MAT_TINT_COMPRESSION: u32 = 15196;

/// Tint A enable | Enable/disable A channel
pub const MAT_TINT_A_ENABLE: u32 = 15198;

/// Tint R specular OPACITY | Tint R specular OPACITY
pub const MAT_TINT_R_SPECULAR_OPACITY: u32 = 15207;

/// Tint G specular OPACITY | Tint G specular OPACITY
pub const MAT_TINT_G_SPECULAR_OPACITY: u32 = 15208;

/// Tint B specular OPACITY | Tint B specular OPACITY
pub const MAT_TINT_B_SPECULAR_OPACITY: u32 = 15209;

/// Tint A specular OPACITY | Tint A specular OPACITY
pub const MAT_TINT_A_SPECULAR_OPACITY: u32 = 15210;

/// Tint R diffuse OPACITY | Tint R diffuse OPACITY
pub const MAT_TINT_R_DIFFUSE_OPACITY: u32 = 15211;

/// Tint G diffuse OPACITY | Tint G diffuse OPACITY
pub const MAT_TINT_G_DIFFUSE_OPACITY: u32 = 15212;

/// Tint B diffuse OPACITY | Tint B diffuse OPACITY
pub const MAT_TINT_B_DIFFUSE_OPACITY: u32 = 15213;

/// Tint A diffuse OPACITY | Tint A diffuse OPACITY
pub const MAT_TINT_A_DIFFUSE_OPACITY: u32 = 15214;

/// Tint Type | Tint Type
pub const MAT_TINT_TYPE: u32 = 15215;

/// Relief map enable | Enable/disable relief map
pub const MAT_RELIEF_MAP_ENABLE: u32 = 15220;

/// Relief map | Texture of the tint map
pub const MAT_RELIEF_MAP: u32 = 15221;

/// Relief map scale | Scale of relief map
pub const MAT_RELIEF_MAP_SCALE: u32 = 15222;

/// Relief map samples | Samples on relief map
pub const MAT_RELIEF_MAP_SAMPLES: u32 = 15223;

/// Relief map offset | Offset for relief map
pub const MAT_RELIEF_MAP_SHADOW_OFFSET: u32 = 15224;

/// Relief map in/out | In/out for relief map
pub const MAT_RELIEF_MAP_IN_OUT: u32 = 15225;

/// Relief map compression | Compression type
pub const MAT_RELIEF_COMPRESSION: u32 = 15226;

/// Tint object exportable | Tint object exportable
pub const MAT_TINT_EXPORTABLE: u32 = 15228;

/// VFX contact sheet width | Contact sheet width
pub const MAT_VFX_CONTACT_SHEET_WIDTH: u32 = 15250;

/// VFX contact sheet height | Contact sheet height
pub const MAT_VFX_CONTACT_SHEET_HEIGHT: u32 = 15251;

/// VFX contact sheet frames | Contact sheet frames
pub const MAT_VFX_CONTACT_SHEET_FRAMES: u32 = 15252;

/// VFX U scroll speed | U scroll speed
pub const MAT_VFX_SCROLL_SPEED_U: u32 = 15253;

/// VFX V scroll speed | V scroll speed
pub const MAT_VFX_SCROLL_SPEED_V: u32 = 15254;

/// VFX Depth bias alpha value | Depth bias alpha value
pub const MAT_VFX_DEPTH_BIAS_ALPHA: u32 = 15255;

/// VFX fresnel end | Alpha fresnel falloff end angle
pub const MAT_VFX_END_ALPHA_FRESNEL: u32 = 15257;

/// Fresnel map enable | Enable/disable of the Fresnel map
pub const MAT_FRESNEL_MAP_ENABLE: u32 = 15280;

/// Fresnel map | Texture file of the Fresnel map
pub const MAT_FRESNEL_MAP: u32 = 15281;

/// Fresnel file name | Texture file used in MAL file
pub const MAT_FRESNEL_FILENAME: u32 = 15282;

/// Emissive map | Texture file of the Emissive map
pub const MAT_EMISSIVE_MAP: u32 = 15311;

/// Emissive file name | Texture file used in MAL file
pub const MAT_EMISSIVE_FILENAME: u32 = 15312;

/// UNUSED
pub const MAT_SECTION_MASK_MAP_ENABLE: u32 = 15340;

/// UNUSED
pub const MAT_SECTION_MASK_MAP: u32 = 15341;

/// UNUSED
pub const MAT_SECTION_MASK_FILENAME: u32 = 15342;

/// UNUSED
pub const MAT_SECTION_MASK_COMPRESSION: u32 = 15343;

/// UNUSED
pub const MAT_SECTION_MASK_COMPRESSION_XBOX360: u32 = 15344;

/// Cornea specular mask | Cornea specular mask
pub const MAT_EYE_CORNEA_SPECULAR_MASK: u32 = 15380;

/// Cornea specular power | Cornea specular power
pub const MAT_EYE_CORNEA_SPECULAR_POWER: u32 = 15381;

/// Sclera specular mask | Sclera specular mask
pub const MAT_EYE_SCLERA_SPECULAR_MASK: u32 = 15382;

/// Sclera specular power | Sclera specular mask
pub const MAT_EYE_SCLERA_SPECULAR_POWER: u32 = 15383;

/// UNUSED
pub const MAT_EYE_CORNEA_REFLECTION_MULTIPLIER: u32 = 15384;

/// UNUSED
pub const MAT_SPECULAR_MASK_MAP_ENABLE: u32 = 15400;

/// Packed Texture map | Texture of the Packed Texture map
pub const MAT_PACKED_TEXTURE_MAP: u32 = 15401;

/// UNUSED
pub const MAT_SPECULAR_SHIFT_MAP_ENABLE: u32 = 15420;

/// Tint Noise map | Texture of the Tint Noise map
pub const MAT_TINT_NOISE_MAP: u32 = 15421;

/// Tint Noise map compression | Compression Type
pub const MAT_TINT_NOISE_COMPRESSION: u32 = 15423;

/// Diffuse Tint | Diffuse Tint color
pub const MAT_HAIR_DIFFUSE_TINT: u32 = 15440;

/// Tint Noise Tiling | Tint Noise Tiling
pub const MAT_HAIR_TINT_NOISE_TILING: u32 = 15445;

/// Sun | Sun object in the material editor
pub const MAT_SUN: u32 = 15460;

/// Sun name | Name of the sun
pub const MAT_SUN_NAME: u32 = 15461;

/// SunlightDirection | vector, the direction of the sunlight
pub const MAT_SUN_DIRECTION: u32 = 15462;

/// SunlightColor | Color, the color of the sunlight
pub const MAT_SUN_COLOR: u32 = 15463;

/// Heraldry map enable | Enable/disable Heraldry map
pub const MAT_HERALDRY_MAP_ENABLE: u32 = 15480;

/// Heraldry map | Texture of the Heraldry map
pub const MAT_HERALDRY_MAP: u32 = 15481;

/// Heraldry map file name | Texture file used in MAL file
pub const MAT_HERALDRY_FILENAME: u32 = 15482;

/// Heraldry map compression | Compression Type
pub const MAT_HERALDRY_COMPRESSION: u32 = 15483;

/// Rim Light Width | Rim Light Width
pub const MAT_RIM_LIGHT_WIDTH: u32 = 15500;

/// Rim Light Multiplier | Rim Light Multiplier
pub const MAT_RIM_LIGHT_MULTIPLIER: u32 = 15501;

/// Falloff Width | Falloff Width
pub const MAT_FALLOFF_WIDTH: u32 = 15502;

/// Falloff Multiplier | Falloff Multiplier
pub const MAT_FALLOFF_MULTIPLIER: u32 = 15503;

/// Ambient Multiplier | Ambient Multiplier
pub const MAT_AMBIENT_MULTIPLIER: u32 = 15510;

/// Specular Multiplier | Specular Multiplier
pub const MAT_SPECULAR_MULTIPLIER: u32 = 15511;

/// Lip Specular Boost | Specular Boost for the lips
pub const MAT_LIP_SPECULAR_BOOST: u32 = 15512;

/// Rim Power | Rim Power
pub const MAT_RIM_POWER: u32 = 15513;

/// Distortion map | Texture file of the Distortion map
pub const MAT_DISTORTION_MAP: u32 = 15521;

/// Distortion file name | Texture file used in MAL file
pub const MAT_DISTORTION_FILENAME: u32 = 15522;

/// Distortion compression for xbox 360 | Compression type of the Distortion map
pub const MAT_DISTORTION_COMPRESSION_XBOX360: u32 = 15524;

/// DistortionModifiers map enable | Enable/disable of the DistortionModifiers map
pub const MAT_DISTORTIONMODIFIERS_MAP_ENABLE: u32 = 15540;

/// DistortionModifiers compression | Compression type of the DistortionModifiers map
pub const MAT_DISTORTIONMODIFIERS_COMPRESSION: u32 = 15543;

/// DistortionModifiers compression for xbox 360 | Compression type of the DistortionModifiers map
pub const MAT_DISTORTIONMODIFIERS_COMPRESSION_XBOX360: u32 = 15544;

/// Distortion magnitude | Distortion magnitude
pub const MAT_DISTORTION_MAGNITUDE: u32 = 15560;

/// Distortion invert | Distortion invert
pub const MAT_DISTORTION_INVERT: u32 = 15561;

/// Alternate decal map | Texture of the diffuse map
pub const MAT_ALTERNATE_DECAL_MAP: u32 = 15580;

/// Tattoo Mask map | Texture of the Tattoo Mask map
pub const MAT_TATTOO_MASK_MAP: u32 = 15590;

/// Tattoo Mask map compression | Compression Type
pub const MAT_TATTOO_MASK_COMPRESSION: u32 = 15592;

/// Brow Stubble map | Texture of the Brow Stubble map
pub const MAT_BROW_STUBBLE_MAP: u32 = 15600;

/// Brow Stubble map compression | Compression Type
pub const MAT_BROW_STUBBLE_COMPRESSION: u32 = 15602;

/// Texture scroll speed | Texture scroll speed
pub const MAT_SCROLL_SPEED_1: u32 = 15650;

/// Texture scroll speed | Texture scroll speed
pub const MAT_SCROLL_SPEED_2: u32 = 15651;

/// Texture scroll speed | Texture scroll speed
pub const MAT_SCROLL_SPEED_3: u32 = 15652;

/// Lava tint color | Lava tint color
pub const MAT_LAVA_TINT_COLOR: u32 = 15653;

/// Lava brightness | Lava brightness
pub const MAT_LAVA_BRIGHTNESS: u32 = 15654;

/// Lava contrast | Lava contrast
pub const MAT_LAVA_CONTRAST: u32 = 15655;

/// Lava noise texture | Lava material noise texture
pub const MAT_LAVA_NOISE_MAP: u32 = 15656;

/// Campaign | Save game Campaign info
pub const SAVEGAME_CAMPAIGN: u32 = 16000;

/// Areas | Area list in save game.
pub const SAVEGAME_AREALIST: u32 = 16001;

/// Party List | The list of party members.
pub const SAVEGAME_PARTYLIST: u32 = 16003;

/// Save game version information | The savegame version information. (The private build number for this save on old saves).
pub const SAVEGAME_VERSION: u32 = 16004;

/// Game State | The state of the game of the current save.
pub const SAVEGAME_GAME_STATE: u32 = 16005;

/// Active Add-Ins | List of all Add-Ins active at save time.
pub const SAVEGAME_ADDINSLIST: u32 = 16006;

/// Save game cheat information | Specifies if a cheat has been used in this save game.
pub const SAVEGAME_CHEAT_USED: u32 = 16007;

/// Placeables | Placeables
pub const SAVEGAME_AREA_PLACEABLES: u32 = 16010;

/// Creatures | Creatures
pub const SAVEGAME_AREA_CREATURES: u32 = 16011;

/// Triggers | Triggers
pub const SAVEGAME_AREA_TRIGGERS: u32 = 16012;

/// Area of Effect Objects | Area of Effect Objects
pub const SAVEGAME_AREA_AOES: u32 = 16013;

/// Waypoints | Waypoints
pub const SAVEGAME_AREA_WAYPOINTS: u32 = 16015;

/// Area Map | Map of the Area
pub const SAVEGAME_AREA_MAP: u32 = 16016;

/// Stores | Stores
pub const SAVEGAME_AREA_STORES: u32 = 16017;

/// Rooms Viewed | Rooms Viewed in the Area
pub const SAVEGAME_AREA_ROOMS_VIEWED: u32 = 16018;

/// Sounds | Sound emitters
pub const SAVEGAME_AREA_SOUNDS: u32 = 16019;

/// Placeables State | Placeable's State
pub const SAVEGAME_AREA_PLACEABLE_STATE: u32 = 16100;

/// Trigger Geometry | Trigger Geometry
pub const SAVEGAME_AREA_TRIGGER_GEOMETRY: u32 = 16101;

/// Placeables useable | Placeable useable
pub const SAVEGAME_AREA_PLACEABLE_USEABLE: u32 = 16102;

/// Trigger sounds list | Trigger sounds list
pub const SAVEGAME_AREA_TRIGGER_SOUNDS: u32 = 16111;

/// Trigger type | Trigger type
pub const SAVEGAME_AREA_TRIGGER_TYPE: u32 = 16112;

/// Store Mark Down | Store Mark Down
pub const SAVEGAME_STORE_MARKDOWN: u32 = 16150;

/// Store Mark Up | Store Mark Up
pub const SAVEGAME_STORE_MARKUP: u32 = 16151;

/// Store Gold | Store Gold
pub const SAVEGAME_STORE_GOLD: u32 = 16152;

/// Store Max Buy Price | Store Max Buy Price
pub const SAVEGAME_STORE_MAXBUYPRICE: u32 = 16153;

/// Store Will-Not-Buy List | Store Will-Not-Buy List
pub const SAVEGAME_STORE_WILLNOTBUY: u32 = 16154;

/// Store Item List | Store Item List
pub const SAVEGAME_STORE_ITEMLIST: u32 = 16156;

/// Party Pool Member Status | Status of the Party Pool Members (active, inactive, etc.).
pub const SAVEGAME_PARTYMEMBERS: u32 = 16203;

/// Party Pool Members | Complete list of members in the party pool (active, inactive, etc.).
pub const SAVEGAME_PARTYPOOLMEMBERS: u32 = 16204;

/// Creature info | Party member creature information.
pub const SAVEGAME_PARTYMEM_CREATURE: u32 = 16205;

/// Template | Party member template.
pub const SAVEGAME_PARTYMEM_TEMPLATE: u32 = 16206;

/// Creature Stats | Creature Stats
pub const SAVEGAME_CREATURE_STATS: u32 = 16209;

/// Backpack | Backpack
pub const SAVEGAME_BACKPACK: u32 = 16210;

/// Plot Items | Plot Items
pub const SAVEGAME_PLOTITEMS: u32 = 16211;

/// Total Money | Total Money
pub const SAVEGAME_MONEY: u32 = 16212;

/// Quick Items | Quick Items
pub const SAVEGAME_QUICKITEMS: u32 = 16213;

/// Equipment | Equipment
pub const SAVEGAME_EQUIPMENT: u32 = 16214;

/// Equipment set | The set of equipment.
pub const SAVEGAME_EQUIPMENTSET: u32 = 16215;

/// Equipment object | Which object is in this slot.
pub const SAVEGAME_EQUIPMENTSET_OBJECT: u32 = 16217;

/// Active Equipment Set | Active Equipment Set
pub const SAVEGAME_EQUIPMENT_ACTIVESET: u32 = 16218;

/// Items List | The items in the equipment set.
pub const SAVEGAME_EQUIPMENT_ITEMS: u32 = 16219;

/// Object Immortal | The object is immortal or not.
pub const SAVEGAME_OBJECT_IMMORTAL: u32 = 16220;

/// Object TAG | The TAG for the object.
pub const SAVEGAME_OBJECT_TAG: u32 = 16222;

/// Items | A list of items.
pub const SAVEGAME_ITEMS: u32 = 16223;

/// Item Droppable | True if the item is droppable
pub const SAVEGAME_ITEM_DROPPABLE: u32 = 16224;

/// Item Damaged | Damaged items cannot be equipped
pub const SAVEGAME_ITEM_DAMAGED: u32 = 16225;

/// Max Inventory Size | Maximum inventory slots
pub const SAVEGAME_MAX_ITEMS: u32 = 16226;

/// Known Crafting Recipes | Known Crafting Recipes
pub const SAVEGAME_CRAFTING_RECIPE_LIST: u32 = 16227;

/// Item Irremovable | Irremovable items cannot be removed by the player once equipped
pub const SAVEGAME_ITEM_IRREMOVABLE: u32 = 16228;

/// Item Indestructible | Indestructible items cannot be destroyed by the player
pub const SAVEGAME_ITEM_INDESTRUCTIBLE: u32 = 16229;

/// Material Type | Dynamic scaling may change item's material using the material progression 2DA
pub const SAVEGAME_ITEM_MATERIALTYPE: u32 = 16230;

/// Item Stealable | True if the item is Stealable
pub const SAVEGAME_ITEM_STEALABLE: u32 = 16231;

/// Object Plot | If the object marked as a plot object.
pub const SAVEGAME_OBJECT_PLOT: u32 = 16250;

/// Object Health | Health of the object.
pub const SAVEGAME_OBJECT_HEALTH: u32 = 16251;

/// Object Max Health | Max health of the object.
pub const SAVEGAME_OBJECT_MAX_HEALTH: u32 = 16252;

/// Object Rank | Rank of the object.
pub const SAVEGAME_OBJECT_RANK: u32 = 16253;

/// Object Name | Non-localized name override.
pub const SAVEGAME_OBJECT_NAME: u32 = 16255;

/// Body bag placeables need this to figure out the dead corpse model name to use
pub const SAVEGAME_OBJECT_LOOTABLE_CREATURE_APPEARANCETYPE: u32 = 16257;

/// Object Importance | Importance of the object to the area and game (used to cut non-essential elements from low end systems)
pub const SAVEGAME_OBJECT_IMPORTANCE: u32 = 16263;

/// GUI status of the Party Picker
pub const SAVEGAME_PARTY_PICKER_GUI_STATUS: u32 = 16274;

/// Party Leader | Leader of the party
pub const SAVEGAME_PARTY_LEADER: u32 = 16278;

/// Party Item Storage Original Owner | Original owner of party item put in storage
pub const SAVEGAME_PARTY_ITEM_STORAGE_OWNER: u32 = 16285;

/// New Item ID | ID number of the new item
pub const SAVEGAME_PARTY_NEW_ITEM_ID: u32 = 16289;

/// Auto level-up default | the auto level-up setting applied to characters as they join the party
pub const SAVEGAME_PARTY_AUTO_LEVEL_DEFAULT: u32 = 16291;

pub const SAVEGAME_PARTY_QUICKBAR_LOCKED: u32 = 16292;

pub const SAVEGAME_PARTY_HOLD_POSITIONS: u32 = 16293;

/// Map zoom level | Area map zoom level
pub const SAVEGAME_PLAYER_MAP_ZOOM: u32 = 16295;

/// Legend | Area map legend visibility
pub const SAVEGAME_PLAYER_MAP_LEGEND: u32 = 16296;

/// Time played | Accumulated time played.
pub const SAVEGAME_PLAYER_TIME_PLAYED: u32 = 16298;

/// Creature Stats Property Non Combat Regen | Creature Stats Property Non Combat Regen
pub const SAVEGAME_STATPROPERTY_REGEN: u32 = 16304;

/// Creature Spell List | Creature Spell List
pub const SAVEGAME_SPELLLIST: u32 = 16305;

/// Creature Talent List | Creature Talent List
pub const SAVEGAME_TALENTLIST: u32 = 16306;

/// Creature Skill List | Creature Skill List
pub const SAVEGAME_SKILLLIST: u32 = 16307;

/// Quickslot Item tag | Name of item linked to the ability in the quickslot (if it exists).
pub const SAVEGAME_QUICKSLOT_ITEMTAG: u32 = 16312;

/// Item Reference Template | A reference to an item template to recover item information if the item was deleted.
pub const SAVEGAME_QUICKSLOT_TEMPLATE: u32 = 16319;

/// Appearance Information | Appearance Information
pub const SAVEGAME_APPEARANCE: u32 = 16320;

/// Appearance Type | Base Appearance Type
pub const SAVEGAME_APPEARANCE_TYPE: u32 = 16321;

/// Appearance Gender | Gender of Appearance
pub const SAVEGAME_APPEARANCE_GENDER: u32 = 16322;

/// Appearance Gore Level | Gore Level of Appearance
pub const SAVEGAME_APPEARANCE_GORE: u32 = 16324;

/// Appearance morph | Morph file
pub const SAVEGAME_APPEARANCE_MORPH_NAME: u32 = 16328;

/// Player portrait pitch
pub const SAVEGAME_PLAYER_PORTRAIT_PITCH: u32 = 16332;

/// Player portrait yaw
pub const SAVEGAME_PLAYER_PORTRAIT_YAW: u32 = 16333;

/// Player portrait tint
pub const SAVEGAME_PLAYER_PORTRAIT_TINT: u32 = 16334;

/// Player portrait expression
pub const SAVEGAME_PLAYER_PORTRAIT_EXPRESSION: u32 = 16335;

/// Player portrait distance from camera
pub const SAVEGAME_PLAYER_PORTRAIT_DISTANCE: u32 = 16336;

/// Player portrait position horizontal
pub const SAVEGAME_PLAYER_PORTRAIT_POSITIONH: u32 = 16337;

/// Player portrait position vertical
pub const SAVEGAME_PLAYER_PORTRAIT_POSITIONV: u32 = 16338;

/// Creature Stats List | Creature Stats List
pub const SAVEGAME_STATLIST: u32 = 16350;

/// The heroic stat list | The heroic stat list for the player and individual followers
pub const SAVEGAME_HEROIC_STATLIST: u32 = 16351;

/// Plot manager | Plot manager
pub const SAVEGAME_PLOT_MANAGER: u32 = 16400;

/// Plot Flag List | Plot Flag List
pub const SAVEGAME_PLOT_LIST: u32 = 16401;

/// Plot Flag GUID | Plot Flag GUID
pub const SAVEGAME_PLOT_GUID: u32 = 16402;

/// Plot Flag Flags 1 | Plot Flag Flags 1
pub const SAVEGAME_PLOT_FLAGS_1: u32 = 16403;

/// Plot Flag Flags 2 | Plot Flag Flags 2
pub const SAVEGAME_PLOT_FLAGS_2: u32 = 16404;

/// Plot Flag Flags 3 | Plot Flag Flags 3
pub const SAVEGAME_PLOT_FLAGS_3: u32 = 16405;

/// Plot Flag Flags 4 | Plot Flag Flags 4
pub const SAVEGAME_PLOT_FLAGS_4: u32 = 16406;

/// Addin UID | Addin UID
pub const SAVEGAME_ADDIN_UID: u32 = 16420;

/// Addin Name ENUS | Addin Name ENUS
pub const SAVEGAME_ADDIN_ENUS: u32 = 16421;

/// Addin Name FRFR | Addin Name FRFR
pub const SAVEGAME_ADDIN_FRFR: u32 = 16422;

/// Addin Name ITIT | Addin Name ITIT
pub const SAVEGAME_ADDIN_ITIT: u32 = 16423;

/// Addin Name DEDE | Addin Name Dutch
pub const SAVEGAME_ADDIN_DEDE: u32 = 16424;

/// Addin Name ESES | Addin Name ESES
pub const SAVEGAME_ADDIN_ESES: u32 = 16425;

/// Addin Name PLPL | Addin Name PLPL
pub const SAVEGAME_ADDIN_PLPL: u32 = 16426;

/// Addin Name RURU | Addin Name RURU
pub const SAVEGAME_ADDIN_RURU: u32 = 16427;

/// Addin Name Pseudo | Addin Name Pseudo
pub const SAVEGAME_ADDIN_PSEUDO: u32 = 16428;

/// Addin Name CSCZ | Addin Name CSCZ
pub const SAVEGAME_ADDIN_CSCZ: u32 = 16429;

/// Addin Name HUHU | Addin Name HUHU
pub const SAVEGAME_ADDIN_HUHU: u32 = 16430;

/// Group List | Group List
pub const SAVEGAME_GROUP_LIST: u32 = 16450;

/// Group ID | Group ID
pub const SAVEGAME_GROUP_ID: u32 = 16451;

/// Group Hostility List | Group Hositlity List
pub const SAVEGAME_GROUP_HOSTILES: u32 = 16452;

/// Team ID | Team ID
pub const SAVEGAME_TEAM_ID: u32 = 16453;

/// Stealth | Creature Stealth state
pub const SAVEGAME_CREATURE_STEALTH: u32 = 16454;

/// Plot giver | Plot giver flag
pub const SAVEGAME_IS_PLOT_GIVER: u32 = 16455;

/// Level up | Creature can level up.
pub const SAVEGAME_CAN_LEVELUP: u32 = 16456;

/// Race | Creature's current race
pub const SAVEGAME_CREATURE_RACE: u32 = 16460;

/// Package | Creature's package
pub const SAVEGAME_CREATURE_PACKAGE: u32 = 16461;

/// Package AI | Creature's package ai
pub const SAVEGAME_CREATURE_PACKAGE_AI: u32 = 16462;

/// Class ID | ID of a class in a class/rank pair
pub const SAVEGAME_CREATURE_CLASS_ID: u32 = 16465;

/// Rank | Ranks in a held class
pub const SAVEGAME_CREATURE_CLASS_RANK: u32 = 16466;

/// List of active modal abilities
pub const SAVEGAME_CREATURE_MODAL_ABILITY_LIST: u32 = 16468;

/// Statue creatures have their animations paused
pub const SAVEGAME_CREATURE_IS_STATUE: u32 = 16470;

/// Minimized talent header list | List of headers minimized in talents/spells GUI
pub const SAVEGAME_CREATURE_MINIMIZED_TALENT_HEADER_LIST: u32 = 16472;

/// Are Equipped items scaled | NPCs items are scaled once based on area and PC level
pub const SAVEGAME_CREATURE_ITEMS_SCALED: u32 = 16474;

/// The roaming radius of the creature
pub const SAVEGAME_CREATURE_ROAM_RADIUS: u32 = 16476;

/// The center of the creature's roaming circle.
pub const SAVEGAME_CREATURE_ROAM_CENTER: u32 = 16477;

/// The name of the pool that a creature belongs to
pub const SAVEGAME_CREATURE_POOL_NAME: u32 = 16478;

/// World Database | World Database
pub const SAVEGAME_WORLDDATABASE: u32 = 16500;

/// Seen Conversation Lines | Lines flagged once per game that have already been seen
pub const SAVEGAME_PARTY_SEEN_LINES: u32 = 16503;

/// Journal | Journal data
pub const SAVEGAME_JOURNAL: u32 = 16504;

/// Journal Title | The name of the quest
pub const SAVEGAME_JOURNAL_TITLE: u32 = 16507;

/// Journal Text | The text of the quest
pub const SAVEGAME_JOURNAL_TEXT: u32 = 16508;

/// Parent Plot | The parent of this plot
pub const SAVEGAME_JOURNAL_PARENT_PLOT: u32 = 16509;

/// Plot ResRef | The ResRef of this plot
pub const SAVEGAME_JOURNAL_RESREF: u32 = 16510;

/// Conversation text | line text
pub const SAVEGAME_JOURNAL_CONVERSATION_LINE_TEXT: u32 = 16519;

/// Coversation reply | player reply
pub const SAVEGAME_JOURNAL_CONVERSATION_LINE_REPLY: u32 = 16520;

/// Unread codex list | unread codex entries
pub const SAVEGAME_JOURNAL_UNREAD_CODEX_LIST: u32 = 16521;

/// Quest Group List | List of quest groups
pub const SAVEGAME_JOURNAL_GROUP_LIST: u32 = 16525;

/// Quest Group Open in Current | Whether this quest group is expanded in current quests
pub const SAVEGAME_JOURNAL_GROUP_OPEN_IN_CURRENT: u32 = 16527;

/// Quest Group Open in Completed | Whether this quest group is expanded in completed quests
pub const SAVEGAME_JOURNAL_GROUP_OPEN_IN_COMPLETED: u32 = 16528;

/// Quest Group Priority | Priority controls the ordering of quest groups in the journal
pub const SAVEGAME_JOURNAL_GROUP_PRIORITY: u32 = 16529;

/// Ambient Dialog Owner | Owner of conversation
pub const SAVEGAME_AMBIENTDIALOG_OWNER: u32 = 16531;

/// Ambient Dialog Speaker | Current speaker
pub const SAVEGAME_AMBIENTDIALOG_SPEAKER: u32 = 16532;

/// Ambient Dialog ResRef | ResRef
pub const SAVEGAME_AMBIENTDIALOG_RESREF: u32 = 16533;

/// Ambient Dialog Line | Current line.
pub const SAVEGAME_AMBIENTDIALOG_LINE: u32 = 16534;

/// Bodybag ID | The ID of the object's bodybag
pub const SAVEGAME_BODYBAG_ID: u32 = 16600;

/// IsBodyBag | True if the placeable is a bodybag
pub const SAVEGAME_ISBODYBAG: u32 = 16601;

/// AoE ID | Type of AoE
pub const SAVEGAME_AOE_ID: u32 = 16603;

/// AoE Shape | Shape of the AoE (Sphere, Rectangle, Cone)
pub const SAVEGAME_AOE_SHAPE: u32 = 16604;

/// AoE Radius | Radius of AoE, for sphere shapes
pub const SAVEGAME_AOE_RADIUS: u32 = 16605;

/// AoE Width | Width of AoE, for rectangle shapes
pub const SAVEGAME_AOE_WIDTH: u32 = 16606;

/// AoE Length | Length of AoE, for rectangle shapes
pub const SAVEGAME_AOE_LENGTH: u32 = 16607;

/// AoE Creator ID | Id Of object which created the AoE
pub const SAVEGAME_AOE_CREATOR: u32 = 16608;

/// AoE Duration | Duration of the AoE
pub const SAVEGAME_AOE_DURATION: u32 = 16609;

/// Creature Rank | creatureranks.xls idx, Exported as 'guibar' from the toolset for some reason.
pub const SAVEGAME_CREATURE_RANK: u32 = 16612;

/// Game Effect ID | ID of the game effect
pub const SAVEGAME_EFFECT_ID: u32 = 16613;

/// Game Effect Type | Type of game effect
pub const SAVEGAME_EFFECT_TYPE: u32 = 16614;

/// Game Effect Duration | Duration of the game effect
pub const SAVEGAME_EFFECT_DURATION: u32 = 16616;

/// Game Effect Sub Type | Sub Type of the game effect
pub const SAVEGAME_EFFECT_SUBTYPE: u32 = 16617;

/// Game Effect Priority | Priority of the game effect
pub const SAVEGAME_EFFECT_PRIORITY: u32 = 16620;

/// Game Effect List | List of game effects
pub const SAVEGAME_EFFECT_LIST: u32 = 16623;

/// Game Effect Engine data | Additional engine specific data for the game effect
pub const SAVEGAME_EFFECT_ENGINE_DATA: u32 = 16624;

/// AI Master | The current AI Master state
pub const SAVEGAME_AI_MASTER: u32 = 16636;

/// AI Event Queue | Queue of the AI events in the AI Master
pub const SAVEGAME_EVENT_QUEUE: u32 = 16630;

/// AI Event Day | Day of the event
pub const SAVEGAME_EVENT_DAY: u32 = 16631;

/// AI Event Time | Time of the event
pub const SAVEGAME_EVENT_TIME: u32 = 16632;

/// AI Event Caller ID | ID of the event caller
pub const SAVEGAME_EVENT_CALLER_ID: u32 = 16633;

/// AI Event Target ID | ID of the event target
pub const SAVEGAME_EVENT_TARGET_ID: u32 = 16634;

/// AI Event ID | ID of the event
pub const SAVEGAME_EVENT_ID: u32 = 16635;

/// Data Arrays | Data Array struct
pub const SAVEGAME_DATAARRAY: u32 = 16640;

/// Data Arrays | Data Arrays integer list
pub const SAVEGAME_DATAARRAY_INT: u32 = 16641;

/// Data Arrays | Data Arrays float list
pub const SAVEGAME_DATAARRAY_FLOAT: u32 = 16642;

/// Data Arrays | Data Arrays bool list
pub const SAVEGAME_DATAARRAY_BOOL: u32 = 16643;

/// Data Arrays | Data Arrays OBJECT_ID list
pub const SAVEGAME_DATAARRAY_OID: u32 = 16644;

/// Data Arrays | Data Arrays string list
pub const SAVEGAME_DATAARRAY_STRING: u32 = 16645;

/// Data Arrays | Data Arrays vector list
pub const SAVEGAME_DATAARRAY_VECTOR: u32 = 16646;

/// Data Arrays | Data Arrays quaternion list
pub const SAVEGAME_DATAARRAY_QUATERNION: u32 = 16647;

/// Event script | Script information for an event
pub const SAVEGAME_EVENT_SCRIPT: u32 = 16650;

/// Script Event Type | Type of the script event
pub const SAVEGAME_SCRIPT_EVENT_TYPE: u32 = 16670;

/// Script Event Target | Target of the script event
pub const SAVEGAME_SCRIPT_EVENT_TARGET: u32 = 16672;

/// Script Event Data | Data of the script event
pub const SAVEGAME_SCRIPT_EVENT_DATA: u32 = 16673;

/// World Timer | World Timer
pub const SAVEGAME_WORLD_TIMER: u32 = 16700;

/// World Timer Day | Day of the World Timer
pub const SAVEGAME_WORLD_TIMER_DAY: u32 = 16701;

/// World Timer Time | Time of the World Timer
pub const SAVEGAME_WORLD_TIMER_TIME: u32 = 16702;

/// Waypoint Map Note Enabled | Enabled status of the map note for this waypoint
pub const SAVEGAME_WAYPOINT_MAPNOTE_ENABLED: u32 = 16711;

/// Waypoint Map Note Loc Text | Localized text of the map note for this waypoint
pub const SAVEGAME_WAYPOINT_MAPNOTE_LOC_TEXT: u32 = 16714;

/// Current Command | Current Command
pub const SAVEGAME_CURRENT_COMMAND: u32 = 16720;

/// Command List | Command List
pub const SAVEGAME_COMMAND_LIST: u32 = 16721;

/// Command Command ID | Command Command ID
pub const SAVEGAME_COMMAND_COMMANDID: u32 = 16722;

/// Command ID | Command ID
pub const SAVEGAME_COMMAND_ID: u32 = 16723;

/// Command Static | Command Static
pub const SAVEGAME_COMMAND_STATIC: u32 = 16724;

/// Command Data | Command Data
pub const SAVEGAME_COMMAND_DATA: u32 = 16725;

/// Player Issued
pub const SAVEGAME_COMMAND_PLAYERISSUED: u32 = 16726;

/// Sub Action List | List of Sub Actions
pub const SAVEGAME_SUBACTION_LIST: u32 = 16730;

/// Sub Action ID | The ID of the Sub Action
pub const SAVEGAME_SUBACTION_ID: u32 = 16731;

/// Sub Action Time Index | Sub Action Time Index
pub const SAVEGAME_SUBACTION_TIME_INDEX: u32 = 16734;

/// Sub Action Length | The ID of the Sub Action
pub const SAVEGAME_SUBACTION_LENGTH: u32 = 16736;

/// Sub Action Start Time | Sub Action Start Time
pub const SAVENAME_SUBACTION_START_TIME: u32 = 16737;

/// Sub Action Data | The data of the Sub Action
pub const SAVEGAME_SUBACTION_DATA: u32 = 16738;

/// AOE Flag | A flag variable for the AOE
pub const SAVEGAME_AOE_FLAGS: u32 = 16751;

/// AOE Stationary | Stationary AoEs mark the pathfinding patches so creatures try to avoid them
pub const SAVEGAME_AOE_STATIONARY: u32 = 16752;

/// Build Number | The private build number from this save.
pub const SAVEGAME_BUILD_NUMBER: u32 = 16770;

/// Internal Save Version Number | The save version number used internally by the development team.
pub const SAVEGAME_SAVE_VERSION_INTERNAL: u32 = 16771;

/// World Map | struct for world map
pub const SAVEGAME_WORLDMAP: u32 = 16780;

/// Primary Map | tag of primary map
pub const SAVEGAME_WORLDMAP_PRIMARYMAP: u32 = 16781;

/// Secondary Map | tag of secondary map
pub const SAVEGAME_WORLDMAP_SECONDARYMAP: u32 = 16782;

/// Map list | list of map objects
pub const SAVEGAME_WORLDMAP_MAPLIST: u32 = 16783;

/// Map tag | tag of map object
pub const SAVEGAME_WORLDMAP_MAP_TAG: u32 = 16784;

/// Pin list | list of map pins
pub const SAVEGAME_WORLDMAP_MAP_PINLIST: u32 = 16786;

/// Pin tag | tag of the map pin
pub const SAVEGAME_WORLDMAP_MAPPIN_TAG: u32 = 16787;

/// Pin state | state of the map pin
pub const SAVEGAME_WORLDMAP_MAPPIN_STATE: u32 = 16788;

/// Last world map pin clicked
pub const SAVEGAME_WORLDMAP_LAST_PIN_CLICKED: u32 = 16791;

/// the pin's previous state
pub const SAVEGAME_WORLDMAP_MAPPIN_LAST_STATE: u32 = 16793;

/// Pin name | name of the map pin
pub const SAVEGAME_WORLDMAP_MAPPIN_NAME: u32 = 16798;

/// Area name | Name of current area.
pub const SAVEGAME_META_AREANAME: u32 = 16800;

/// Time played | Seconds played.
pub const SAVEGAME_META_TIMEPLAYED: u32 = 16801;

/// Level | Hero's level
pub const SAVEGAME_META_LEVEL: u32 = 16802;

/// Class | Hero's class
pub const SAVEGAME_META_CLASS: u32 = 16803;

/// Gender | Hero's gender
pub const SAVEGAME_META_GENDER: u32 = 16804;

/// Race | Hero's race
pub const SAVEGAME_META_RACE: u32 = 16805;

/// Background | Hero's background
pub const SAVEGAME_META_BACKGROUND: u32 = 16806;

/// Name | Hero's name
pub const SAVEGAME_META_NAME: u32 = 16807;

/// Savegame name | Save name
pub const SAVEGAME_META_SAVENAME: u32 = 16808;

/// Tactic target object ID | Object ID of the object that the tactic target refers to
pub const SAVEGAME_TACTICENTRY_TARGET_OBJECT_ID: u32 = 16818;

/// Tactic condition object ID | Object ID of the object that the tactic condition refers to
pub const SAVEGAME_TACTICENTRY_CONDITION_OBJECT_ID: u32 = 16819;

/// Has tactics table | Creature has a tactics table
pub const SAVEGAME_TACTICS_HAS_TABLE: u32 = 16821;

/// Tactics table | struct for tactics table
pub const SAVEGAME_TACTICS_TABLE: u32 = 16822;

/// Tactics enabled | tactics enabled for creature
pub const SAVEGAME_TACTICS_ENABLED: u32 = 16823;

/// Tactics list | list of tactics entries
pub const SAVEGAME_TACTICS_LIST: u32 = 16824;

/// Entry enabled | Tactic entry is enabled
pub const SAVEGAME_TACTICENTRY_ENABLED: u32 = 16825;

/// Tactic target | Target of the tactic
pub const SAVEGAME_TACTICENTRY_TARGET: u32 = 16826;

/// Tactic command | Command for the tactic
pub const SAVEGAME_TACTICENTRY_COMMAND: u32 = 16828;

/// Tactic condition tag | Tag of the object that the tactic condition refers to
pub const SAVEGAME_TACTICENTRY_CONDITIONTAG: u32 = 16831;

/// Tactics dirty
pub const SAVEGAME_TACTICS_DIRTY: u32 = 16832;

/// Tactics preset type
pub const SAVEGAME_TACTICS_PRESETTYPE: u32 = 16833;

/// Tactics preset index
pub const SAVEGAME_TACTICS_PRESETINDEX: u32 = 16834;

/// Tactics preset list
pub const SAVEGAME_TACTICS_PRESETLIST: u32 = 16835;

/// Tactics custom list
pub const SAVEGAME_TACTICS_CUSTOMLIST: u32 = 16836;

pub const SAVEGAME_TACTICENTRY_COMMANDITEMTAG: u32 = 16837;

pub const SAVEGAME_TACTICENTRY_COMMANDITEMRESREF: u32 = 16838;

/// Plot actions | Plot actions control
pub const SAVEGAME_PLOTACTIONS: u32 = 16840;

/// Plot actions enabled | enabled state
pub const SAVEGAME_PLOTACTIONS_ENABLED: u32 = 16841;

/// Current set | current action set
pub const SAVEGAME_PLOTACTIONS_CURRENTSET: u32 = 16842;

/// Actions list | list of all plot actions
pub const SAVEGAME_PLOTACTIONS_LIST: u32 = 16843;

/// Plot action id | Unique identifier of plot action
pub const SAVEGAME_PLOTACTION_ID: u32 = 16844;

/// Plot action state | current state of plot action
pub const SAVEGAME_PLOTACTION_STATE: u32 = 16845;

/// Sound object tag | sound object tag
pub const SAVEGAME_SOUND_TAG: u32 = 16900;

/// Sound active flag | is sound active
pub const SAVEGAME_SOUND_ACTIVE: u32 = 16901;

/// Sound event name | sound event name
pub const SAVEGAME_SOUND_NAME: u32 = 16902;

/// sound position X | sound position X
pub const SAVEGAME_SOUND_XPOSITION: u32 = 16903;

/// sound position Y | sound position Y
pub const SAVEGAME_SOUND_YPOSITION: u32 = 16904;

/// sound position Z | sound position Z
pub const SAVEGAME_SOUND_ZPOSITION: u32 = 16905;

/// sound orientation X | sound orientation X
pub const SAVEGAME_SOUND_XORIENTATION: u32 = 16906;

/// sound orientation Y | sound orientation Y
pub const SAVEGAME_SOUND_YORIENTATION: u32 = 16907;

/// sound orientation Z | sound orientation Z
pub const SAVEGAME_SOUND_ZORIENTATION: u32 = 16908;

/// sound orientation W | sound orientation W
pub const SAVEGAME_SOUND_WORIENTATION: u32 = 16909;

/// sound volume | sound volume
pub const SAVEGAME_SOUND_VOLUME: u32 = 16910;

/// sound pitch | sound pitch
pub const SAVEGAME_SOUND_PITCH: u32 = 16911;

/// sound fade in | sound fade in
pub const SAVEGAME_SOUND_FADEIN: u32 = 16912;

/// sound fade out | sound fade out
pub const SAVEGAME_SOUND_FADEOUT: u32 = 16913;

/// sound inside cone | sound inside cone
pub const SAVEGAME_SOUND_CONEINSIDE: u32 = 16915;

/// sound outside cone | sound outside cone
pub const SAVEGAME_SOUND_CONEOUTSIDE: u32 = 16916;

/// sound cone volume | sound cone volume
pub const SAVEGAME_SOUND_CONEVOLUME: u32 = 16917;

/// sound priority | sound priority
pub const SAVEGAME_SOUND_PRIORITY: u32 = 16918;

/// sound occludable flag | is sound occludable flag
pub const SAVEGAME_SOUND_OCCLUDABLE: u32 = 16919;

/// Head morph for main player
pub const SAVEGAME_PLAYER_MORPH: u32 = 16950;

/// current player creature soundset
pub const SAVEGAME_PLAYER_SOUNDSET: u32 = 16951;

/// default (from GFF) player creature soundset
pub const SAVEGAME_DEFAULT_SOUNDSET: u32 = 16952;

/// Add-In Name | Add-In Name
pub const SAVEGAME_ADDIN_NAME: u32 = 16960;

/// Story So Far Event List | List of Events tracked in the hero's story so far.
pub const SAVEGAME_STORYSOFAR_EVENTLIST: u32 = 16970;

/// Story So Far Event ID | The event Id for each event tracked in the hero's story (so far).
pub const SAVEGAME_STORYSOFAR_EVENTID: u32 = 16971;

/// Story So Far Game Time | The In Game Time that the Story So Far event occurred at.
pub const SAVEGAME_STORYSOFAR_GAMETIME: u32 = 16972;

/// Story So Far UTC | The real world time that the STory So Far event occurred at.
pub const SAVEGAME_STORYSOFAR_UTC: u32 = 16973;

/// Story So Far Screen Shot | The screen shot (if any) associated with this event Id for the hero's story (so far).
pub const SAVEGAME_STORYSOFAR_SCREENSHOT: u32 = 16974;

/// Story So Far Level Up Stats | The Level Up stats of the hero's at each level up event.
pub const SAVEGAME_STORYSOFAR_LEVELUPLIST: u32 = 16975;

/// Story So Far Area Name for Level Up Stats | The Area that the hero was in during level up.
pub const SAVEGAME_STORYSOFAR_AREA: u32 = 16976;

/// Story So Far Current Health | The hero's current health at each level up event.
pub const SAVEGAME_STORYSOFAR_CURRENT_HEATLH: u32 = 16979;

/// Story So Far Current Stamina | The hero's current stamina at each level up event.
pub const SAVEGAME_STORYSOFAR_CURRENT_STAMINA: u32 = 16981;

/// Story So Far Total Stamina | The hero's total stamina at each level up event.
pub const SAVEGAME_STORYSOFAR_TOTAL_STAMINA: u32 = 16982;

/// Story So Far Current XP | The hero's current experisnce points at each level up event.
pub const SAVEGAME_STORYSOFAR_CURRENT_XP: u32 = 16983;

/// Story So Far Base Attribute | The hero's base attribute at each level up event.
pub const SAVEGAME_STORYSOFAR_ATTRIBUTE_BASE: u32 = 16988;

/// Story So Attribute Modifier | The hero's attribute modifier at each level up event.
pub const SAVEGAME_STORYSOFAR_ATTRIBUTE_MODIFIER: u32 = 16989;

pub const SAVEGAME_STORYSOFAR_EQUIPMENT_LIST: u32 = 16990;

pub const SAVEGAME_STORYSOFAR_EQUIPMENT_SLOTID: u32 = 16991;

pub const SAVEGAME_STORYSOFAR_EQUIPMENT_RESREF: u32 = 16992;

pub const SAVEGAME_STORYSOFAR_EQUIPMENT_STACKSIZE: u32 = 16993;

pub const SAVEGAME_STORYSOFAR_ITEM_PROPERTY: u32 = 16994;

pub const SAVEGAME_STORYSOFAR_ITEM_POWER: u32 = 16995;

pub const SAVEGAME_STORYSOFAR_ITEM_DATA: u32 = 16996;

/// VarTable | struct list, Script Var Table
pub const SCRIPTVARTABLE: u32 = 17000;

/// Name | WCHAR, VarTable Entry Name
pub const SCRIPTVARTABLE_NAME: u32 = 17001;

/// Type | BYTE, VarTable Entry Type
pub const SCRIPTVARTABLE_TYPE: u32 = 17002;

/// Value | VarTable Entry Value
pub const SCRIPTVARTABLE_VALUE: u32 = 17003;

/// Entry Area | Area that the player starts in
pub const CAMPAIGN_CIF_ENTRY_AREA: u32 = 17101;

pub const CAMPAIGN_CIF_DISPLAY_NAME_FR_FR: u32 = 17107;

pub const CAMPAIGN_CIF_DISPLAY_NAME_DE_DE: u32 = 17108;

pub const CAMPAIGN_CIF_DISPLAY_NAME_PL_PL: u32 = 17109;

pub const CAMPAIGN_CIF_DISPLAY_NAME_RU_RU: u32 = 17110;

pub const CAMPAIGN_CIF_DISPLAY_NAME_IT_IT: u32 = 17111;

pub const CAMPAIGN_CIF_DISPLAY_NAME_ES_ES: u32 = 17112;

pub const CAMPAIGN_CIF_DISPLAY_NAME_HU_HU: u32 = 17113;

pub const CAMPAIGN_CIF_DISPLAY_NAME_CS_CZ: u32 = 17114;

/// Description | The localized description
pub const CAMPAIGN_CIF_DESCRIPTION_EN_US: u32 = 17115;

pub const CAMPAIGN_CIF_DESCRIPTION_FR_FR: u32 = 17116;

pub const CAMPAIGN_CIF_DESCRIPTION_DE_DE: u32 = 17117;

pub const CAMPAIGN_CIF_DESCRIPTION_PL_PL: u32 = 17118;

pub const CAMPAIGN_CIF_DESCRIPTION_RU_RU: u32 = 17119;

pub const CAMPAIGN_CIF_DESCRIPTION_IT_IT: u32 = 17120;

pub const CAMPAIGN_CIF_DESCRIPTION_ES_ES: u32 = 17121;

pub const CAMPAIGN_CIF_DESCRIPTION_HU_HU: u32 = 17122;

pub const CAMPAIGN_CIF_DESCRIPTION_CS_CZ: u32 = 17123;

/// Bucket List | List of buckets in the hash table
pub const TALK_BUCKET_LIST: u32 = 19000;

/// String List | List of strings inside each bucket
pub const TALK_STRING_LIST: u32 = 19001;

/// String ID | The string ID of the string
pub const TALK_STRING_ID: u32 = 19002;

/// String | The string value of the string
pub const TALK_STRING: u32 = 19003;

/// States List | List of state patches
pub const PLACEABLE_STATES_LIST: u32 = 20000;

/// Child list | VFX editor child list
pub const VFX_CHILD_LIST: u32 = 21000;

/// VFX Object ID | VFX Object ID
pub const VFX_OBJECT_ID: u32 = 21001;

/// Emitter Parameter | Emitter Parameter
pub const VFX_EMITTER_INITIALROTATIONRANGE: u32 = 21002;

/// Root | Root object in VFX editor
pub const VFX_ROOT: u32 = 21004;

/// Roll Axis| Roll axis for chunky particles
pub const VFX_EMITTER_MESH_PARTICLE_ROLL_AXIS: u32 = 21005;

/// VFX Type | The type of vfx this vfx is
pub const VFX_TYPE: u32 = 21006;

/// VFX Render Object Visible | VFX Render Object Visible
pub const VFX_OBJECT_VISIBLE: u32 = 21007;

/// Keyframe | Keyframe in a list of animation values
pub const VFX_KEYFRAME: u32 = 21009;

/// Value | Value for a given keyframe
pub const VFX_VALUE: u32 = 21010;

/// Emitter | Emitter object in VFX editor
pub const VFX_EMITTER_NAME: u32 = 21011;

/// Emitter Type | Type of emitter
pub const VFX_EMITTER_TYPE: u32 = 21012;

/// Emitter Parameter | Emitter Parameter
pub const VFX_EMITTER_UPDATEONLYWHENVISIBLE: u32 = 21014;

/// Emitter Parameter | Emitter Parameter
pub const VFX_EMITTER_LINKPARTICLESTOGETHER: u32 = 21015;

/// Emitter Parameter | Emitter Parameter
pub const VFX_EMITTER_MATERIALLIBRARY: u32 = 21016;

/// Emitter Parameter | Emitter Parameter
pub const VFX_EMITTER_MATERIALOBJECT: u32 = 21017;

/// Emitter Parameter | Emitter Parameter
pub const VFX_EMITTER_BIRTHRATE: u32 = 21018;

/// Emitter Parameter | Emitter Parameter
pub const VFX_EMITTER_BIRTHRATERANGE: u32 = 21019;

/// Emitter Parameter | Emitter Parameter
pub const VFX_EMITTER_INITIALSPEED: u32 = 21021;

/// Emitter Parameter | Emitter Parameter
pub const VFX_EMITTER_INITIALSPEEDRANGE: u32 = 21022;

/// Emitter Parameter | Emitter Parameter
pub const VFX_EMITTER_ACCELERATION: u32 = 21023;

/// Emitter Parameter | Emitter Parameter
pub const VFX_EMITTER_GRAVITYMULTIPLIER: u32 = 21024;

/// Emitter Parameter | Emitter Parameter
pub const VFX_EMITTER_LIFE: u32 = 21025;

/// Emitter Parameter | Emitter Parameter
pub const VFX_EMITTER_LIFERANGE: u32 = 21026;

/// Emitter Parameter | Emitter Parameter
pub const VFX_EMITTER_SCALERANGE: u32 = 21027;

/// Emitter Parameter | Emitter Parameter
pub const VFX_EMITTER_SPREADX: u32 = 21028;

/// Emitter Parameter | Emitter Parameter
pub const VFX_EMITTER_SPREADY: u32 = 21029;

/// Emitter Parameter | Emitter Parameter
pub const VFX_EMITTER_INITIALROTATIONSPEED: u32 = 21030;

/// Emitter Parameter | Emitter Parameter
pub const VFX_EMITTER_INITIALROTATIONSPEEDRANGE: u32 = 21031;

/// Emitter Parameter | Emitter Parameter
pub const VFX_EMITTER_ROTATIONALACCELERATION: u32 = 21032;

/// Deprecated March 20/08.  Left in for backwards-compatibility with VFXPROJ files -PjW
pub const VFX_EMITTER_RANDOMINITIALROTATION: u32 = 21033;

/// Emitter Parameter | Emitter Parameter
pub const VFX_EMITTER_PARTICLEINHERITANCE: u32 = 21034;

/// Emitter Parameter | Emitter Parameter
pub const VFX_EMITTER_PARTICLESAFFECTEDBYWIND: u32 = 21036;

/// Emitter Parameter | Emitter Parameter
pub const VFX_EMITTER_ENABLEPARTICLECOLLISIONS: u32 = 21037;

/// Emitter Parameter | Emitter Parameter
pub const VFX_EMITTER_PHYSICSOBJECTSPAWN: u32 = 21038;

/// Emitter Parameter | Emitter Parameter
pub const VFX_EMITTER_PHYSICSEMITTER: u32 = 21039;

/// Emitter Parameter | Emitter Parameter
pub const VFX_EMITTER_MOVEMENTSPREADX: u32 = 21040;

/// Emitter Parameter | Emitter Parameter
pub const VFX_EMITTER_MOVEMENTSPREADY: u32 = 21041;

/// Emitter Parameter | Emitter Parameter
pub const VFX_EMITTER_MOVEMENTSPREADUPDATEDELAY: u32 = 21042;

/// Emitter Parameter | Emitter Parameter
pub const VFX_EMITTER_TARGETNAME: u32 = 21043;

/// Emitter Parameter | Emitter Parameter
pub const VFX_EMITTER_TARGETATTRACTION: u32 = 21044;

/// Emitter Parameter | Emitter Parameter
pub const VFX_EMITTER_TARGETRADIUS: u32 = 21045;

/// Emitter Parameter | Emitter Parameter
pub const VFX_EMITTER_SPAWNDIRECTIONTRACKSTARGET: u32 = 21046;

/// Emitter Parameter | Emitter Parameter
pub const VFX_EMITTER_KILLPARTICLEWHENTARGETHIT: u32 = 21047;

/// Emitter Parameter | Emitter Parameter
pub const VFX_EMITTER_PARTICLESFOLLOWPATH: u32 = 21048;

/// Emitter Flipbook Parameter | Flipbook type
pub const VFX_EMITTER_FLIPBOOK_TYPE: u32 = 21049;

/// Emitter Flipbook Parameter | Flipbook Rows
pub const VFX_EMITTER_FLIPBOOK_ROWS: u32 = 21051;

/// Emitter Flipbook Parameter | Flipbook Columns
pub const VFX_EMITTER_FLIPBOOK_COLUMNS: u32 = 21052;

/// Emitter Parameter | Emitter Parameter
pub const VFX_EMITTER_ALPHAMULTIPLIER: u32 = 21054;

/// Emitter Parameter | Emitter Parameter
pub const VFX_EMITTER_COLORMULTIPLIER: u32 = 21055;

/// Emitter Parameter | Emitter Parameter
pub const VFX_EMITTER_SCALEMULTIPLIER: u32 = 21056;

/// Emitter Parameter | Emitter Parameter
pub const VFX_EMITTER_INFINITELIFE: u32 = 21057;

/// Emitter Parameter | Emitter Parameter
pub const VFX_EMITTER_CHUNKY_MODEL_NAME: u32 = 21058;

/// Emitter Parameter | Emitter Parameter
pub const VFX_EMITTER_INITIALROTATION: u32 = 21059;

/// Crust Node | Crust Node object in VFX editor
pub const VFX_CRUSTNODE_NAME: u32 = 21060;

/// Crust Node ID | The crust hook ID value
pub const VFX_CRUSTNODE_CRUSTHOOKID: u32 = 21062;

/// Geometry File Name | External Geometry File Name
pub const VFX_GEOMETRY_FILE_NAME: u32 = 21063;

/// Flag | Flag
pub const VFX_USE_VARIATION_TINT: u32 = 21065;

/// Dummy Node Name | Dummy object in VFX editor
pub const VFX_DUMMY_NAME: u32 = 21070;

/// Geometry Name | Geometry object in VFX editor
pub const VFX_GEOMETRY_NAME: u32 = 21080;

/// Geometry scale | scale of geometry
pub const VFX_GEOMETRY_SCALE: u32 = 21081;

/// Target Name | Target object in VFX editor
pub const VFX_TARGET_NAME: u32 = 21090;

/// Model Name | Reference model object in VFX editor
pub const VFX_MODEL_NAME: u32 = 21100;

/// Creature Name | Reference crust based  object in VFX editor
pub const VFX_CREATURE_NAME: u32 = 21110;

/// Creature URI | Reference crust based  object in VFX editor
pub const VFX_CREATURE_URI: u32 = 21111;

/// Position X | The x coordinate of the position
pub const VFX_RELATIVE_POSITION_X: u32 = 21120;

/// Position Y | The Y coordinate of the position
pub const VFX_RELATIVE_POSITION_Y: u32 = 21121;

/// Position Z | The Z coordinate of the position
pub const VFX_RELATIVE_POSITION_Z: u32 = 21122;

/// Impact | Length of Impact animation
pub const VFX_IMPACT_LENGTH: u32 = 21130;

/// Duration | Length of Duration animation
pub const VFX_DURATION_LENGTH: u32 = 21131;

/// Cessation | Length of Cessation animation
pub const VFX_CESSATION_LENGTH: u32 = 21132;

/// Custom | Length of Custom animation
pub const VFX_CUSTOM_LENGTH: u32 = 21133;

/// Custom Name | Name of Custom animation
pub const VFX_CUSTOM_NAME: u32 = 21134;

/// Age Map Red | Age Map red color
pub const VFX_AGEMAP_COLOR_R: u32 = 21140;

/// Age Map Green | Age Map green color
pub const VFX_AGEMAP_COLOR_G: u32 = 21141;

/// Age Map Blue | Age Map blue color
pub const VFX_AGEMAP_COLOR_B: u32 = 21142;

/// Age Map Alpha | Age Map alpha
pub const VFX_AGEMAP_COLOR_A: u32 = 21143;

/// Age Map Scale X | Age Map X scale
pub const VFX_AGEMAP_SCALE_X: u32 = 21144;

/// Age Map Scale Y | Age Map Y scale
pub const VFX_AGEMAP_SCALE_Y: u32 = 21145;

/// Emitter Event | Emitter Event
pub const VFX_EVENT: u32 = 21150;

/// Event Time | Emitter Event
pub const VFX_EVENT_TIME: u32 = 21151;

/// Event Type | Emitter Event
pub const VFX_EVENT_TYPE: u32 = 21152;

/// Event ID | Emitter Event
pub const VFX_EVENT_ID: u32 = 21153;

/// Target System | Emitter Event
pub const VFX_EVENT_TARGETSYSTEM: u32 = 21154;

/// Emitter Parameter | Emitter Parameter
pub const VFX_EMITTER_VOLUME_SPAWN_TYPE: u32 = 21160;

/// Collision Type | Collision Type
pub const VFX_EMITTER_COLLISION_TYPE: u32 = 21163;

/// Collision Bounce Value | Collision Bounce Value
pub const VFX_EMITTER_BOUNCE_VALUE: u32 = 21164;

/// Emitter Parameter | Emitter Parameter
pub const VFX_EMITTER_VOLUME_SPAWN_WITHIN_VOLUME: u32 = 21165;

/// Emitter Parameter | Emitter Parameter
pub const VFX_EMITTER_VOLUME_SPAWN_INVERT_NORMALS: u32 = 21166;

/// Emitter Parameter | Emitter Parameter
pub const VFX_EMITTER_COLORMULTIPLIER_R: u32 = 21170;

/// Emitter Parameter | Emitter Parameter
pub const VFX_EMITTER_COLORMULTIPLIER_G: u32 = 21171;

/// Emitter Parameter | Emitter Parameter
pub const VFX_EMITTER_COLORMULTIPLIER_B: u32 = 21172;

/// Splat Age Map Red | Splat Age Map red color
pub const VFX_SPLAT_AGEMAP_COLOR_R: u32 = 21173;

/// Splat Age Map Green | Splat Age Map green color
pub const VFX_SPLAT_AGEMAP_COLOR_G: u32 = 21174;

/// Splat Age Map Blue | Splat Age Map blue color
pub const VFX_SPLAT_AGEMAP_COLOR_B: u32 = 21175;

/// Splat Age Map Alpha | Splat Age Map alpha
pub const VFX_SPLAT_AGEMAP_COLOR_A: u32 = 21176;

/// Splat Age Map Scale X | Splat Age Map X scale
pub const VFX_SPLAT_AGEMAP_SCALE_X: u32 = 21177;

/// Splat Age Map Scale Y | Splat Age Map Y scale
pub const VFX_SPLAT_AGEMAP_SCALE_Y: u32 = 21178;

/// VFX File Vesion | VFX File Version
pub const VFX_FILE_OBJECT_VERSION: u32 = 21180;

/// Emitter Parameter | Emitter Parameter
pub const VFX_EMITTER_SPLAT_ALPHAMULTIPLIER: u32 = 21181;

/// Emitter Parameter | Emitter Parameter
pub const VFX_EMITTER_SPLAT_COLORMULTIPLIER_R: u32 = 21182;

/// Emitter Parameter | Emitter Parameter
pub const VFX_EMITTER_SPLAT_COLORMULTIPLIER_G: u32 = 21183;

/// Emitter Parameter | Emitter Parameter
pub const VFX_EMITTER_SPLAT_COLORMULTIPLIER_B: u32 = 21184;

/// Emitter Parameter | Emitter Parameter
pub const VFX_EMITTER_WORLD_AXIS_ACCELERATION_X: u32 = 21193;

/// Emitter Parameter | Emitter Parameter
pub const VFX_EMITTER_WORLD_AXIS_ACCELERATION_Y: u32 = 21194;

/// Emitter Parameter | Emitter Parameter
pub const VFX_EMITTER_WORLD_AXIS_ACCELERATION_Z: u32 = 21195;

/// VFX Range(Bounding Box) | VFX Range(Bounding Box)
pub const VFX_RANGE: u32 = 21196;

/// Emitter Parameter | Emitter Parameter
pub const VFX_EMITTER_AXIS_ACCELERATION_SPACE: u32 = 21197;

/// Emitter Parameter | Emitter Parameter
pub const VFX_EMITTER_UVDISTRIBUTIONSIZE: u32 = 21198;

/// Emitter Group | Emitter group name
pub const VFX_EMITTER_GROUP_NAME: u32 = 21210;

pub const VFX_REMOTE_MATERIAL_TINT_R: u32 = 21220;

pub const VFX_REMOTE_MATERIAL_TINT_G: u32 = 21221;

pub const VFX_REMOTE_MATERIAL_TINT_B: u32 = 21222;

pub const VFX_REMOTE_MATERIAL_TINT_A: u32 = 21223;

pub const VFX_REMOTE_MATERIAL_FRESNEL_FALLOFF: u32 = 21224;

pub const VFX_REMOTE_MATERIAL_INVERT_FRESNEL: u32 = 21225;

pub const VFX_REMOTE_MATERIAL_ALPHA: u32 = 21226;

pub const VFX_REMOTE_MATERIAL_DECAL_NAME: u32 = 21227;

/// WND file root
pub const WND_ROOT: u32 = 22000;

/// Resref | used in ARL
pub const WND_RESREF: u32 = 22001;

/// float | radius of effect
pub const WND_RADIUS: u32 = 22002;

/// float | Strengh [0,1]
pub const WND_STRENGTH: u32 = 22003;

/// Vec3 | Direction
pub const WND_DIRECTION: u32 = 22004;

/// float | falloff for local effects
pub const WND_RADIUS_FALLOFF: u32 = 22005;

/// The gusting min strength [0,1]
pub const WND_GUST_MIN_STRENGTH: u32 = 22010;

/// The gusting max strength [0,1]
pub const WND_GUST_MAX_STRENGTH: u32 = 22011;

/// The gusting min duration
pub const WND_GUST_MIN_DURATION: u32 = 22012;

/// The gusting max strength
pub const WND_GUST_MAX_DURATION: u32 = 22013;

/// float | freq Hz
pub const WND_GUST_FREQUENCY: u32 = 22014;

/// float | number of leaf angles to use in rendering
pub const WND_TREE_NUM_LEAF_ANGLES: u32 = 22021;

/// float | wind response
pub const WND_TREE_RESPONSE: u32 = 22022;

/// float | response limit
pub const WND_TREE_RESPONSE_LIMIT: u32 = 22023;

/// float | maximum bending in degrees
pub const WND_TREE_MAX_BEND_ANGLE: u32 = 22024;

/// float | SPT branch exponent
pub const WND_TREE_BRANCH_EXPONENT: u32 = 22025;

/// float | SPT leaf exponent
pub const WND_TREE_LEAF_EXPONENT: u32 = 22026;

/// vec4 | from speedtree cad
pub const WND_TREE_BRANCH_OSCILLATION_X: u32 = 22027;

/// vec4 | from speedtree cad
pub const WND_TREE_BRANCH_OSCILLATION_Y: u32 = 22028;

/// vec4 | from speedtree cad
pub const WND_TREE_LEAF_ROCKING: u32 = 22029;

/// vec4 | from speedtree cad
pub const WND_TREE_LEAF_RUSTLING: u32 = 22030;

/// float | Maximum cloth response to wind
pub const WND_CLOTH_RESPONSE: u32 = 22031;

/// float | Cloth wind strength
pub const WND_CLOTH_STRENGTH: u32 = 22033;

/// float | Minimum gusting strength for cloth
pub const WND_CLOTH_GUST_STRENGTH_MIN: u32 = 22034;

/// float | Maximum gusting strength for cloth
pub const WND_CLOTH_GUST_STRENGTH_MAX: u32 = 22035;

/// float | Minimum duration of gusting for cloth
pub const WND_CLOTH_GUST_DURATION_MIN: u32 = 22036;

/// float | Maximum duration of gusting for cloth
pub const WND_CLOTH_GUST_DURATION_MAX: u32 = 22037;

/// list of ATMO
pub const ATMO_DATA: u32 = 22500;

/// vector3 | Sun color for atmosphere.
pub const ATMO_SUN_COLOR: u32 = 22519;

/// float | Turbidity factor for MIE term
pub const ATMO_TURBIDITY: u32 = 22521;

/// float | Extinction term (Mie) multiplier
pub const ATMO_EARTH_REFLECTANCE: u32 = 22522;

/// float | Modulates the Mie scattering term
pub const ATMO_MIE_MULTIPLIER: u32 = 22523;

/// float | Modulates the rayleigh term
pub const ATMO_RAYLEIGH_MULTIPLIER: u32 = 22524;

/// float | Modulates the in-scatter
pub const ATMO_EARTH_IN_SCATTER_POWER: u32 = 22525;

/// float | Henyey / Greenstein phase eccentricity
pub const ATMO_PHASE_ECCENTRICITY: u32 = 22527;

/// float | Amount of atmospheric influence over the level
pub const ATMO_ALPHA: u32 = 22528;

/// Vec3f | Distance-based fog color
pub const ATMO_FOG_COLOR: u32 = 22529;

/// float | Distance-based fog intensity
pub const ATMO_FOG_INTENSITY: u32 = 22530;

/// float | Maximum fog index
pub const ATMO_FOG_CAP: u32 = 22531;

/// float | Aperture angle for skybox (vertical) fog
pub const ATMO_FOG_ZENITH: u32 = 22532;

/// float | Distance-based intensity for water planes
pub const ATMO_FOG_WATER_INTENSITY: u32 = 22533;

/// float | Maximum fog index for water planes
pub const ATMO_FOG_WATER_CAP: u32 = 22534;

/// list of CLDS
pub const CLOUD_DATA: u32 = 22600;

/// float | Overcast -> scatter
pub const CLOUD_DENSITY: u32 = 22620;

/// float | Controls the thickness of clouds
pub const CLOUD_SHARPNESS: u32 = 22621;

/// float | Controls the virtual heightmap offset.
pub const CLOUD_DEPTH: u32 = 22622;

/// float | Multiplies the cloud texture coords.
pub const CLOUD_RANGE_MULTIPLIER1: u32 = 22623;

/// float | Multiplies the cloud texture coords.
pub const CLOUD_RANGE_MULTIPLIER2: u32 = 22624;

/// vector3 | Cloud tinting color.
pub const CLOUD_COLOR: u32 = 22625;

/// float | Scale for the moon (or sun) in the sky
pub const MOON_SCALE: u32 = 22700;

/// float | Alpha for the moon in the sky
pub const MOON_ALPHA: u32 = 22701;

/// float | Alpha for the moon as reflected by the clouds
pub const MOON_CLOUDALPHA: u32 = 22702;

/// float | Rotation for moon (or sun or fade tear) in the sky
pub const MOON_ROTATION: u32 = 22703;

/// Morph parts | Parts composing the morphed head
pub const MORPH_PARTS: u32 = 23000;

/// Tints | Tint file names
pub const MORPH_TINTFILENAMES: u32 = 23001;

/// Nodes | List of morphed nodes
pub const MORPH_NODES: u32 = 23002;

/// Texture | Texture name for material parameter
pub const MORPH_TEXTURE_NAME: u32 = 23003;

/// Textures | Texture material parameter list
pub const MORPH_TEXTUREPARAM: u32 = 23004;

/// Vectors | Vector material parameter list
pub const MORPH_VECTOR4FPARAM: u32 = 23005;

/// Floats | Float material parameter list
pub const MORPH_FLOATPARAM: u32 = 23006;

/// Float | Float material parameter value
pub const MORPH_FLOATPARAMVALUE: u32 = 23007;

/// String | Morph name
pub const MORPH_NAME: u32 = 23008;

/// String | Node name for material
pub const MORPH_MAT_NODE_NAME: u32 = 23009;

/// String | Parameter name for material
pub const MORPH_MAT_PARAM_NAME: u32 = 23010;

/// Int | Parameter's index for material
pub const MORPH_MAT_PARAM_INDEX: u32 = 23011;

/// Float | Parameter's value for material
pub const MORPH_MAT_PARAM_VALUE: u32 = 23012;

/// vector4 | Parameter's vector for material
pub const MORPH_MAT_PARAM_VECTOR: u32 = 23013;

/// Struct list | Material's parameters list
pub const MORPH_MAT_PARAMS: u32 = 23014;

/// Struct list | Material's parameters list
pub const MORPH_MAT_VEC_PARAMS: u32 = 23015;

/// String | Model's name
pub const MORPH_MODEL_NAME: u32 = 23016;

/// Float32 | Model's parameter value
pub const MORPH_MODEL_VALUE: u32 = 23017;

/// Struct list | Model's parameters list
pub const MORPH_MODEL_PARAMS: u32 = 23018;

/// String | Node name for texture
pub const MORPH_TEX_NODE_NAME: u32 = 23019;

/// String | Parameter name for texture
pub const MORPH_TEX_PARAM_NAME: u32 = 23020;

/// String | Texture name
pub const MORPH_TEX_NAME: u32 = 23021;

/// Struct list | Textures
pub const MORPH_TEXTURES: u32 = 23022;

/// Tag | Tag name of a map
pub const MAP_TAG: u32 = 24000;

/// Type | 2DA index of the map type
pub const MAP_TYPE: u32 = 24001;

/// Pin list | List of map pins
pub const MAP_PINLIST: u32 = 24002;

/// State | State of the map pin
pub const MAP_PIN_STATE: u32 = 24003;

/// X | x position of map pin
pub const MAP_PIN_POS_X: u32 = 24004;

/// Y | y position of map pin
pub const MAP_PIN_POS_Y: u32 = 24005;

/// Name | Name of map pin
pub const MAP_PIN_NAME: u32 = 24006;

/// Tag | Tag of map pin
pub const MAP_PIN_TAG: u32 = 24007;

/// Area tag | Tag of associated area
pub const MAP_PIN_AREATAG: u32 = 24008;

/// Terrain | Terrain type of area
pub const MAP_PIN_TERRAINTYPE: u32 = 24009;

/// Type | Type of map pin
pub const MAP_PIN_TYPE: u32 = 24010;

/// Maps | List of map objects
pub const MAP_MAPS: u32 = 24011;

/// Trail list | List of map trails
pub const MAP_TRAILLIST: u32 = 24014;

/// Pin 1 Tag | Pin 1's tag
pub const MAP_TRAIL_PIN_1_TAG: u32 = 24015;

/// Pin 2 Tag | Pin 1's tag
pub const MAP_TRAIL_PIN_2_TAG: u32 = 24016;

/// Point list | List of map trail points
pub const MAP_POINTLIST: u32 = 24017;

/// X | x position of map trail point
pub const MAP_POINT_POS_X: u32 = 24018;

/// Y | y position of map trail point
pub const MAP_POINT_POS_Y: u32 = 24019;

/// Tooltip | Tooltip of map pin
pub const MAP_PIN_TOOLTIP: u32 = 24020;

/// FileList | list of files+dependencies
pub const DEP_FILE_LIST: u32 = 25000;

/// Name | string, resref of file
pub const DEP_RESREF: u32 = 25001;

/// DependencyList | string list of dependencies
pub const DEP_DEPENDENCY_LIST: u32 = 25002;

/// MOP structure
pub const CHAR_MOP: u32 = 250100;

/// Appearance structure
pub const CHAR_APP: u32 = 250101;

/// Gender
pub const CHAR_GENDER: u32 = 250102;

/// Race
pub const CHAR_RACE: u32 = 250103;

/// Class
pub const CHAR_CLASS: u32 = 250104;

/// Background
pub const CHAR_BACK: u32 = 250105;

/// List of attributes
pub const CHAR_ATTRIBUTES: u32 = 250106;

/// List of abilities
pub const CHAR_ABILITIES: u32 = 250107;

/// Character name
pub const CHAR_NAME: u32 = 250108;

/// Character's head name
pub const CHAR_HEAD_NAME: u32 = 250109;

/// Attribute ID
pub const CHAR_ATTRIBUTE_ID: u32 = 250110;

/// Attribute points
pub const CHAR_ATTRIBUTE_POINTS: u32 = 250111;

/// Portrait data
pub const CHAR_PORTRAIT: u32 = 250112;

/// Build Number | Build number during last save event.
pub const SAVEPROFILE_BUILD_NUMBER: u32 = 26000;

/// Profile List | List of Different account Profiles.
pub const SAVEPROFILE_PROFILELIST: u32 = 26003;

/// Achievement ID | The ID of this Achievement.
pub const SAVEPROFILE_ACHIEVEMENT_ID: u32 = 26007;

/// New Achievement | Indicates if this achievement should appear as newly unlocked on the gui.
pub const SAVEPROFILE_ACHIEVEMENT_NEW: u32 = 26008;

/// Online Achievement | Indicates if this achievement should appears as online on the gui.
pub const SAVEPROFILE_ACHIEVEMENT_ONLINE: u32 = 26009;

/// Addin List | List of AddIns
pub const SAVEPROFILE_ADDIN_LIST: u32 = 26100;

/// Offer List | List of Offers
pub const SAVEPROFILE_OFFER_LIST: u32 = 26101;

/// Content Name | The name of the add-in
pub const SAVEPROFILE_CONTENT_NAME: u32 = 26102;

/// Content Enabled | The enable status of the add-in
pub const SAVEPROFILE_CONTENT_ENABLED: u32 = 26104;

/// Content File Data | Additional data needed to open a protected file in an add-in
pub const SAVEPROFILE_FILE_DATA: u32 = 26109;

/// Content File Version | Version of the add-in
pub const SAVEPROFILE_FILE_VERSION: u32 = 26110;

/// Addin Token List | List of AddIn Tokens
pub const SAVEPROFILE_ADDIN_TOKEN_LIST: u32 = 26111;

pub fn field_name_by_id(id: u32) -> Option<&'static str> {
    match id {
        0 => Some("INVALIDENTRY"),
        1 => Some("TAG"),
        2 => Some("NAME"),
        3 => Some("TEMPLATERESREF"),
        4 => Some("POSITION"),
        5 => Some("ORIENTATION"),
        6 => Some("UINT8_LIST"),
        7 => Some("INT8_LIST"),
        8 => Some("UINT16_LIST"),
        9 => Some("INT16_LIST"),
        10 => Some("UINT32_LIST"),
        11 => Some("INT32_LIST"),
        12 => Some("UINT64_LIST"),
        13 => Some("INT64_LIST"),
        14 => Some("FLOAT32_LIST"),
        15 => Some("FLOAT64_LIST"),
        16 => Some("VECTOR3F_LIST"),
        17 => Some("VECTOR4F_LIST"),
        18 => Some("QUATERNIONF_LIST"),
        19 => Some("ECSTRING_LIST"),
        20 => Some("COLOR4F_LIST"),
        21 => Some("NAME_HASH"),
        22 => Some("TEXT"),
        23 => Some("OBJECT_ID"),
        900 => Some("TS_PROPERTY"),
        901 => Some("TS_PROPERTY_NAME"),
        902 => Some("TS_PROPERTY_ATOM"),
        903 => Some("TS_PROPERTY_VALUE"),
        904 => Some("TS_PROPERTY_CHILDREN"),
        905 => Some("TS_PROPERTY_VARTYPE"),
        1000 => Some("ITEM_BASEID"),
        1001 => Some("ITEM_COST"),
        1002 => Some("ITEM_STACKSIZE"),
        1003 => Some("ITEM_STOLEN"),
        1004 => Some("ITEM_PLOT"),
        1005 => Some("ITEM_IDENTIFIED"),
        1006 => Some("ITEM_CHARGES"),
        1007 => Some("ITEM_MODELVARIATION"),
        1008 => Some("ITEM_DESCRIPTION"),
        1009 => Some("ITEM_PROPERTYLIST"),
        1010 => Some("ITEM_MATERIAL"),
        1011 => Some("ITEM_ABILITYID"),
        1012 => Some("ITEM_ABILITYPWR"),
        1013 => Some("ITEM_PROPERTIES"),
        1014 => Some("ITEM_PROPERTY_POWERS"),
        1015 => Some("ITEM_PROPERTY_EFFECTID"),
        1018 => Some("ITEM_PROPERTY_VFXID"),
        1019 => Some("ITEM_SUBITEMS_RESREFS"),
        1020 => Some("ITEM_CRAFTINGRECIPEID"),
        1021 => Some("ITEM_BASECOST"),
        2000 => Some("ITEM_PROP_PARAM1"),
        2001 => Some("ITEM_PROP_PROPERTYNAME"),
        2002 => Some("ITEM_PROP_SUBTYPE"),
        2003 => Some("ITEM_PROP_COSTTABLE"),
        2004 => Some("ITEM_PROP_COSTVALUE"),
        2005 => Some("ITEM_PROP_PARAM1VALUE"),
        2006 => Some("ITEM_PROP_CHANCEAPPEAR"),
        3000 => Some("ENV_WORLD"),
        3001 => Some("ENV_WORLD_NAME"),
        3002 => Some("ENV_WORLD_AREA_LIST"),
        3003 => Some("LVL_CHILD_LIST"),
        3004 => Some("LVL_FILE_OBJECT_VERSION"),
        3005 => Some("LVL_CHANGETIME"),
        3010 => Some("ENV_AREA"),
        3011 => Some("ENV_AREA_ID"),
        3012 => Some("ENV_AREA_NAME"),
        3013 => Some("ENV_AREA_FILE"),
        3014 => Some("ENV_AREA_ENVIRONMENTSETTINGS"),
        3015 => Some("ENV_AREA_NAVIGATION_INFO_FILE"),
        3016 => Some("ENV_AREA_ROOM_LIST"),
        3017 => Some("ENV_AREA_ROOM_LIST_ELEMENT"),
        3018 => Some("ENV_AREA_POSITION"),
        3019 => Some("ENV_AREA_ROTATION"),
        3020 => Some("ENV_AREA_PATHFINDING_EXPORT"),
        3021 => Some("ENV_AREA_PATHFINDING_VISINFO"),
        3022 => Some("ENV_AREA_PATHFINDING_VISINFO_COUNT"),
        3023 => Some("ENV_AREA_FRAME_BUFFER_EFFECT"),
        3024 => Some("ENV_AREA_CENTER"),
        3025 => Some("ENV_AREA_SKYDOME_MODEL"),
        3026 => Some("ENV_AREA_FRAME_BUFFER_EFFECT_LIST"),
        3027 => Some("ENV_AREA_GLOBALWIND_RESREF"),
        3028 => Some("ENV_AREA_LOCALWIND_LIST"),
        3029 => Some("ENV_AREA_PATHFINDING_COSTS"),
        3030 => Some("ENV_ROOM"),
        3031 => Some("ENV_ROOM_ID"),
        3032 => Some("ENV_ROOM_NAME"),
        3033 => Some("ENV_ROOM_FILE"),
        3034 => Some("ENV_ROOM_ENVIRONMENTSETTINGS"),
        3035 => Some("ENV_ROOM_POSITION"),
        3036 => Some("ENV_ROOM_ROTATION"),
        3037 => Some("ENV_ROOM_PATHFINDING_GRIDSEPARATION"),
        3038 => Some("ENV_ROOM_PATHFINDING_CHARACTERHEIGHT"),
        3039 => Some("ENV_ROOM_PATHFINDING_CLEARANCE"),
        3040 => Some("ENV_ROOM_PATHFINDING_EXPORT"),
        3041 => Some("ENV_ROOM_PATHFINDING_VISINFO"),
        3042 => Some("ENV_ROOM_PATHFINDING_VISINFO_COUNT"),
        3043 => Some("ENV_ROOM_PATH_GRID_FILE"),
        3044 => Some("ENV_ROOM_PATHCONNECTION_LIST"),
        3045 => Some("ENV_ROOM_PATHCONNECTION"),
        3046 => Some("ENV_ROOM_PATHCONNECTION_ID"),
        3047 => Some("ENV_ROOM_VISIBILITY_LIST"),
        3048 => Some("ENV_ROOM_VISIBILITY"),
        3049 => Some("ENV_ROOM_VISIBILITY_ID"),
        3050 => Some("ENV_ROOM_MODEL_LIST"),
        3051 => Some("ENV_ROOM_LIGHT_LIST"),
        3052 => Some("ENV_ROOM_MODEL_LIST_ELEMENT"),
        3053 => Some("ENV_ROOM_LIGHT_LIST_ELEMENT"),
        3054 => Some("ENV_ROOM_DYNSHADOW_DIRECTION"),
        3055 => Some("ENV_ROOM_DYNSHADOW_ENABLED"),
        3056 => Some("ENV_MODEL_PATHFINDING_OVERLAPPED"),
        3057 => Some("ENV_MODEL_SHOW_HIGH_LOD"),
        3058 => Some("ENV_MODEL_SNAP_TO_TERRAIN"),
        3059 => Some("ENV_MODEL_SCALE"),
        3060 => Some("ENV_MODEL"),
        3061 => Some("ENV_MODEL_ID"),
        3062 => Some("ENV_MODEL_NAME"),
        3063 => Some("ENV_MODEL_FILE"),
        3064 => Some("ENV_MODEL_POSITION"),
        3065 => Some("ENV_MODEL_ROTATION"),
        3066 => Some("ENV_MODEL_PATHFINDING_NORMAL"),
        3067 => Some("ENV_LIGHT"),
        3068 => Some("ENV_LIGHT_ID"),
        3069 => Some("ENV_LIGHT_NAME"),
        3070 => Some("ENV_LIGHT_POSITION"),
        3071 => Some("ENV_LIGHT_ROTATION"),
        3072 => Some("LIGHT_COLOR"),
        3073 => Some("LIGHT_ISDYNAMIC"),
        3074 => Some("LIGHT_TYPE"),
        3075 => Some("LIGHT_POINT_RADIUS"),
        3076 => Some("LIGHT_COLOR_MULTIPLIER"),
        3077 => Some("LIGHT_BAKED"),
        3078 => Some("LIGHT_EFFECT"),
        3079 => Some("LIGHT_AFFECT_DOMAIN"),
        3080 => Some("AREAGRID_NAVINFO"),
        3081 => Some("AREAGRID_ROOMNAME"),
        3082 => Some("AREAGRID_GRIDNAVINFO"),
        3083 => Some("AREAGRID_MODELGRID"),
        3084 => Some("AREAGRID_GRIDID"),
        3085 => Some("AREAGRID_MODELID"),
        3086 => Some("AREAGRID_NBCOLUMNS"),
        3087 => Some("AREAGRID_NBROWS"),
        3088 => Some("AREAGRID_CELLSIZE"),
        3089 => Some("AREAGRID_CLEARANCE"),
        3090 => Some("AREAGRID_BASEPOS"),
        3091 => Some("AREAGRID_NORMAL"),
        3092 => Some("AREAGRID_DATA"),
        3093 => Some("AREAGRID_HEIGHT"),
        3094 => Some("AREAGRID_ABSTRACTION_SECTORSIZE"),
        3095 => Some("AREAGRID_ABSTRACTION_SECTORS"),
        3096 => Some("AREAGRID_ABSTRACTION_MEMORY"),
        3097 => Some("AREAGRID_ID"),
        3098 => Some("AREAGRID_CELLID"),
        3099 => Some("ENV_ROOM_CONNECTIVITY_LIST"),
        3100 => Some("LIGHT_SPOT_INNER_ANGLE"),
        3101 => Some("LIGHT_SPOT_OUTER_ANGLE"),
        3102 => Some("LIGHT_SPOT_DISTANCE"),
        3103 => Some("ENV_LIGHT_PROBE"),
        3104 => Some("ENV_LIGHT_PROBE_ENVMAP"),
        3105 => Some("ENV_LIGHT_NUM_SAMPLES"),
        3106 => Some("ENV_LIGHT_SIZE"),
        3107 => Some("DYNAMICSHADOW_VECTOR_GAME"),
        3108 => Some("ENV_LIGHT_PROBE_ID"),
        3109 => Some("ENV_MODEL_CUT_AWAY_OVERRIDE"),
        3110 => Some("AREAGRID_AREA"),
        3114 => Some("AREAGRID_SOUND_DATA"),
        3115 => Some("AREAGRID_ABSTRACTION_SNUMREG"),
        3116 => Some("AREAGRID_ABSTRACTION_SADDR"),
        3117 => Some("AREAGRID_LIGHT_SUBSET_DATA8"),
        3118 => Some("AREAGRID_LIGHT_SUBSET_DATA16"),
        3119 => Some("LIGHT_CAN_BE_OCCLUDED"),
        3120 => Some("AREAGRID_CELLPADDING"),
        3122 => Some("ENV_AREA_CHUNK_ISCHUNK"),
        3123 => Some("ENV_AREA_CHUNK_ROWCOUNT"),
        3124 => Some("ENV_AREA_CHUNK_COLCOUNT"),
        3125 => Some("ENV_AREA_CHUNK_WIDTH"),
        3126 => Some("ENV_AREA_CHUNK_HEIGHT"),
        3127 => Some("ENV_AREA_LAYOUT_NAME"),
        3128 => Some("ENV_AREA_STARTPOINT_NAME"),
        3129 => Some("ENV_AREA_CUTOFF_HEIGHT"),
        3130 => Some("LIGHT_ANIMATED_MIN_FREQUENCY"),
        3131 => Some("LIGHT_ANIMATED_MAX_FREQUENCY"),
        3132 => Some("LIGHT_ANIMATED_MIN_INTENSITY"),
        3133 => Some("LIGHT_ANIMATED_MAX_INTENSITY"),
        3134 => Some("ENV_AREA_CUTOFF_SYSTEM_ENABLED"),
        3137 => Some("ENV_MINIMAP_TEXTURE_MAP_COORDS"),
        3138 => Some("ENV_MINIMAP_LOWER_LEFT_POINT"),
        3139 => Some("ENV_MINIMAP_UPPER_RIGHT_POINT"),
        3140 => Some("ENV_ROOM_LOWER_LEFT_POINT"),
        3141 => Some("ENV_ROOM_UPPER_RIGHT_POINT"),
        3142 => Some("ENV_AREA_FORCE_CHARACTER_LIGHTING"),
        3148 => Some("ENV_AREA_SUNLIGHT_CAN_BE_OCCLUDED_CHAR"),
        3149 => Some("ENV_AREA_SUNLIGHT_COLOR_CHAR"),
        3150 => Some("ENV_AREA_SUNLIGHT_DIRECTION"),
        3151 => Some("ENV_AREA_SUNLIGHT_ENABLED"),
        3152 => Some("ENV_AREA_SUNLIGHT_COLOR"),
        3153 => Some("ENV_AREA_SUNLIGHT_COLORMULT"),
        3154 => Some("TERRAIN_CHUNK"),
        3155 => Some("TERRAIN_CHUNK_LIST"),
        3156 => Some("TERRAIN_CHUNK_CELL_POSITION_X"),
        3157 => Some("TERRAIN_CHUNK_CELL_POSITION_Y"),
        3158 => Some("TERRAIN_CHUNK_LENGTH"),
        3159 => Some("TERRAIN_CHUNK_WIDTH"),
        3160 => Some("TERRAIN_CHUNK_TEXEL_SIZE"),
        3161 => Some("TERRAIN_CHUNK_BLENDPAGE_SIZE"),
        3162 => Some("TERRAIN_CHUNK_SECTOR_ID"),
        3164 => Some("ENV_ROOM_LIGHT_VIS_LIST"),
        3165 => Some("ENV_FOG_COLOR"),
        3166 => Some("ENV_FOG_MAX_DISTANCE"),
        3167 => Some("ENV_FOG_MAX_INTENSITY"),
        3168 => Some("ENV_FOG_ENABLED"),
        3169 => Some("ENV_FOG_MIN_DISTANCE"),
        3170 => Some("ENV_MODEL_NAME_CHANGED"),
        3171 => Some("ENV_VEGETATION"),
        3172 => Some("ENV_CREATURE"),
        3200 => Some("ENV_CAMERA"),
        3201 => Some("ENV_CAMERA_PIVOTDISTANCE"),
        3202 => Some("ENV_LIST_AREA"),
        3203 => Some("ENV_LIST_ROOM"),
        3204 => Some("ENV_LIST_MODEL"),
        3205 => Some("ENV_LIST_LIGHT"),
        3210 => Some("ENV_PFCONTAINER_LAYOUTNAME"),
        3211 => Some("ENV_PFCONTAINER_EXPORTDATA"),
        3212 => Some("ENV_PFCONTAINER_DATAVERSION"),
        3213 => Some("ENV_PFCONTAINER_VISINFO"),
        3235 => Some("ENV_MODEL_LIGHTMAP_PART_ID"),
        3290 => Some("RIMTREE_ROOT_NODE"),
        3291 => Some("RIMTREE_RIM_LIST"),
        3292 => Some("RIMTREE_CHILD_LIST"),
        3293 => Some("RIMTREE_NODE_TAG"),
        3294 => Some("RIMTREE_NODE_RESREF"),
        3300 => Some("ENV_GROUP"),
        3301 => Some("ENV_GROUP_NAME"),
        3302 => Some("ENV_SP_GROUP"),
        3303 => Some("ENV_SP_GROUP_NAME"),
        3304 => Some("ENV_SP"),
        3305 => Some("ENV_SP_FILE"),
        3310 => Some("ENV_OBJECT_VISIBLE"),
        3311 => Some("ENV_OBJECT_LOCKSELECTION"),
        3320 => Some("ENV_MODEL_INSTANCEID"),
        3321 => Some("ENV_MODEL_BOUNDS_CENTER"),
        3322 => Some("ENV_MODEL_BOUNDS_RADIUS"),
        3323 => Some("ENV_MODEL_LIGHTMAP_ATLAS"),
        3324 => Some("ENV_MODEL_LIGHTMAP_OFFSET_SCALE"),
        3326 => Some("ENV_MODEL_LIGHTMAP_ATLAS_LIST"),
        3330 => Some("LVL_LIGHTMAP_SIZE_MULTIPLIER"),
        3331 => Some("LVL_LIGHTMAP_LAST_UPDATED_LIST"),
        3332 => Some("LVL_LIGHTMAP_LAST_UPDATED"),
        3333 => Some("LVL_LIGHTMAP_FILESPEC"),
        3334 => Some("LVL_LIGHTING_VERSION"),
        3340 => Some("LVL_AO_COLOR_MIN"),
        3341 => Some("LVL_AO_COLOR_MAX"),
        3342 => Some("LVL_AO_SAMPLES_MIN"),
        3343 => Some("LVL_AO_SAMPLES_MAX"),
        3344 => Some("LVL_AO_ADAPTSAMPLEENABLED"),
        3345 => Some("LVL_AO_ADAPTSAMPLEACCURACY"),
        3346 => Some("LVL_AO_ADAPTSAMPLESMOOTH"),
        3347 => Some("LVL_AO_CONEANGLE"),
        3348 => Some("LVL_AO_MAXRAYLENGTH"),
        3349 => Some("LVL_AO_EXPONENT"),
        3350 => Some("ENV_TREE"),
        3351 => Some("ENV_TREENODE_ID"),
        3352 => Some("ENV_TREE_NAME"),
        3353 => Some("ENV_TREE_FILE"),
        3354 => Some("ENV_ROOM_TREENODE_LIST"),
        3355 => Some("ENV_AREA_TREECONTROLLER_LIST"),
        3356 => Some("ENV_TREE_SCALE"),
        3357 => Some("ENV_AREA_TREECONTROLLER_ID"),
        3358 => Some("ENV_TREE_PAINTED_LIST"),
        3359 => Some("ENV_TREE_PAINTED_POSITION"),
        3360 => Some("ENV_TREE_PAINTED_ROTATION"),
        3361 => Some("ENV_TREE_PAINTED_SCALE"),
        3362 => Some("ENV_SCATTER_OBJECTS"),
        3363 => Some("ENV_SCATTEROBJECT_FILE"),
        3364 => Some("ENV_SCATTER_INSTANCE"),
        3365 => Some("ENV_SCATTER_INSTANCE_LIST"),
        3366 => Some("ENV_SCATTEROBJECT_LIST"),
        3367 => Some("ENV_SCATTEROBJECT_ID"),
        3368 => Some("ENV_SCATTEROBJ_IGNORE_MAX_DENSITY"),
        3369 => Some("ENV_SCATTEROBJ_MAX_DENSITY"),
        3370 => Some("ENV_SCATTEROBJ_MIN_SCALE"),
        3371 => Some("ENV_SCATTEROBJ_MAX_SCALE"),
        3372 => Some("ENV_SCATTEROBJ_ORIENT"),
        3373 => Some("ENV_SCATTEROBJ_PROTOTYPE"),
        3374 => Some("ENV_SCATTEROBJ_MSI_DATA"),
        3375 => Some("ENV_TREE_COLOR_TINT"),
        3376 => Some("ENV_SCATTEROBJ_SOUND_TYPE"),
        3377 => Some("ENV_TREE_COLOR_LEVEL_TINT"),
        3378 => Some("ENV_TREE_COLOR_LEVEL_INTENSITY"),
        3379 => Some("ENV_TREE_DRAW_DISTANCE"),
        3400 => Some("TERRAIN_EXPORT_AREA"),
        3401 => Some("TERRAIN_EXPORT_AREA_LIST"),
        3403 => Some("TERRAIN_AREA_CELL_POSITION_X"),
        3404 => Some("TERRAIN_AREA_CELL_POSITION_Y"),
        3405 => Some("TERRAIN_AREA_CELL_POSITION_Z"),
        3406 => Some("TERRAIN_AREA_CELL_SIZE_X"),
        3407 => Some("TERRAIN_AREA_CELL_SIZE_Y"),
        3408 => Some("TERRAIN_AREA_CELL_SIZE_Z"),
        3409 => Some("TERRAIN_AREA_BORDER_CELL_WIDTH"),
        3410 => Some("TERRAIN_AREA_VISTA_CELL_WIDTH"),
        3411 => Some("TERRAIN_AREA_LIGHTMAP_SIZE"),
        3412 => Some("TERRAIN_AREA_LIGHTMAP_SIZE_VISTA"),
        3413 => Some("TERRAIN_AREA_SUBDIVIDE_BY"),
        3500 => Some("ENV_MODEL_PARTGROUP"),
        3501 => Some("ENV_MODEL_LIGHTMAPONLY"),
        3502 => Some("ENV_MODEL_LIGHTMAP_FLAG"),
        3503 => Some("ENV_MODEL_EXPORT_FLAG"),
        3504 => Some("ENV_MODEL_DEFAULT_ANIMATION"),
        3505 => Some("ENV_MODEL_BLEND_TREE_NAME"),
        3506 => Some("ENV_MODEL_USER_PARAM_LIST"),
        3507 => Some("ENV_MODEL_USER_PARAM_NAME"),
        3508 => Some("ENV_MODEL_USER_PARAM_VALUE"),
        3600 => Some("LVL_WATER"),
        3601 => Some("LVL_WATER_SIZE_X"),
        3602 => Some("LVL_WATER_SIZE_Y"),
        3603 => Some("LVL_WATER_MAX_TESSELLATION"),
        3604 => Some("LVL_WATER_MESH_ID"),
        3605 => Some("LVL_WATER_NORMAL_MAP"),
        3606 => Some("LVL_WATER_HEIGHT_MAP"),
        3607 => Some("LVL_WATER_DEEP_COLOR"),
        3608 => Some("LVL_WATER_SHALLOW_COLOR"),
        3609 => Some("LVL_WATER_WAVE_FREQ_1"),
        3610 => Some("LVL_WATER_WAVE_AMPL_1"),
        3611 => Some("LVL_WATER_WAVE_DIRECTION_1"),
        3612 => Some("LVL_WATER_WAVE_FREQ_2"),
        3613 => Some("LVL_WATER_WAVE_AMPL_2"),
        3614 => Some("LVL_WATER_WAVE_DIRECTION_2"),
        3615 => Some("LVL_WATER_WAVE_FREQ_3"),
        3616 => Some("LVL_WATER_WAVE_AMPL_3"),
        3617 => Some("LVL_WATER_WAVE_DIRECTION_3"),
        3618 => Some("LVL_WATER_WAVE_SPEED_1"),
        3619 => Some("LVL_WATER_WAVE_SPEED_2"),
        3620 => Some("LVL_WATER_WAVE_SPEED_3"),
        3621 => Some("LVL_WATER_REFLECTIVITY"),
        3622 => Some("LVL_WATER_FOAM_HEIGHT"),
        3623 => Some("LVL_WATER_SUBDIVISION_DEPTH_TOLERANCE"),
        3624 => Some("LVL_WATER_SHALLOW_DEPTH"),
        3625 => Some("LVL_WATER_FOAM_COLOR"),
        3626 => Some("LVL_WATER_WALKABLE_DEPTH"),
        3627 => Some("LVL_WATER_WALL_HEIGHT"),
        3628 => Some("LVL_WATER_OPACITY_FALLOFF"),
        3629 => Some("LVL_WATER_SUNLIGHT_SPECULAR_POWER"),
        3630 => Some("LVL_WATER_SPECULAR_MULTIPLIER"),
        3631 => Some("LVL_WATER_SPECULAR_FALLOFF"),
        3632 => Some("LVL_WATER_COLORIZE_TRANSPARENCY"),
        3633 => Some("LVL_WATER_OVERRIDE_REFLECTION"),
        3634 => Some("LVL_WATER_ENABLE_SPEC"),
        3700 => Some("LVL_WIND"),
        3701 => Some("LVL_WIND_ID"),
        3702 => Some("LVL_WIND_NAME"),
        3710 => Some("LVL_WIND_ISGLOBAL"),
        3711 => Some("LVL_WIND_REGIONRADIUS"),
        3712 => Some("LVL_WIND_REGIONFALLOFF"),
        3713 => Some("LVL_WIND_SPTSTRENGTH"),
        3714 => Some("LVL_WIND_SPTGUST_MINPERCENT"),
        3715 => Some("LVL_WIND_SPTGUST_MAXPERCENT"),
        3716 => Some("LVL_WIND_SPTGUST_MINDURATION"),
        3717 => Some("LVL_WIND_SPTGUST_MAXDURATION"),
        3718 => Some("LVL_WIND_SPTBENDANGLE"),
        3719 => Some("LVL_WIND_CLOTH_RESPONSE"),
        3720 => Some("LVL_WIND_CLOTH_RESPONSE_LMT"),
        3721 => Some("LVL_WIND_CLOTH_STRENGTH"),
        3722 => Some("LVL_WIND_CLOTH_GUST_STRENGTH_MIN"),
        3723 => Some("LVL_WIND_CLOTH_GUST_STRENGTH_MAX"),
        3724 => Some("LVL_WIND_CLOTH_GUST_DURATION_MIN"),
        3725 => Some("LVL_WIND_CLOTH_GUST_DURATION_MAX"),
        3726 => Some("LVL_WIND_CLOTH_GUST_INTERVAL_MIN"),
        3727 => Some("LVL_WIND_CLOTH_GUST_INTERVAL_MAX"),
        3728 => Some("LVL_WIND_CLOTH_GUST_DIR_CHANGE"),
        3729 => Some("LVL_WIND_CLOTH_GUST_AXIS_RATIO"),
        3730 => Some("LVL_COLLISION_WALL_INFO"),
        3731 => Some("LVL_COLLISION_WALL_VERTICIES"),
        3732 => Some("LVL_COLLISION_WALL_VERTICIES_V2"),
        3740 => Some("LVL_MINIMAP_POSITION_X"),
        3741 => Some("LVL_MINIMAP_POSITION_Y"),
        3742 => Some("LVL_MINIMAP_SIZE_X"),
        3743 => Some("LVL_MINIMAP_SIZE_Y"),
        3744 => Some("ENV_STAT_PHYS"),
        3745 => Some("ENV_STAT_PHYS_DATA"),
        3800 => Some("LVL_LIGHT_SUBSET_LIST"),
        3801 => Some("LVL_LIGHT_SUBSET_ENTRY"),
        3802 => Some("LVL_LIGHT_SUBSET_TOTAL_ENTRIES"),
        4000 => Some("ANIMATION_NODENAME"),
        4001 => Some("ANIMATION_TARGET"),
        4002 => Some("ANIMATION_SOURCETYPE"),
        4003 => Some("ANIMATION_ELEMENTSPERENTRY"),
        4004 => Some("ANIMATION_NODEDATA"),
        4005 => Some("ANIMATION_NODELIST"),
        4006 => Some("ANIMATION_NAME"),
        4007 => Some("ANIMATION_GENERALANIMNAME"),
        4008 => Some("ANIMATION_HASGOBANIM"),
        4009 => Some("ANIMATION_ANIMLENGTH"),
        4010 => Some("ANIMATION_COMBATRANGE"),
        4011 => Some("ANIMATION_ISADDITIVE"),
        4012 => Some("ANIMATION_ISOVERRIDE"),
        4013 => Some("ANIMATION_OVERRIDEPRIORITY"),
        4014 => Some("ANIMATION_NAME_HASH"),
        4015 => Some("ANIMATION_NODENAME_HASH"),
        4016 => Some("ANIMATION_EVENT_TIME"),
        4017 => Some("ANIMATION_EVENT_ID"),
        4018 => Some("ANIMATION_EVENT_TARGET"),
        4019 => Some("ANIMATION_EVENT_STRING"),
        4020 => Some("ANIMATION_EVENT_LIST"),
        4021 => Some("ANIMATION_TREE"),
        4022 => Some("ANIMATION_TREE_NAME"),
        4023 => Some("ANIMATION_TREE_NODE"),
        4024 => Some("ANIMATION_TREE_NODE_NAME"),
        4025 => Some("ANIMATION_TREE_NODE_FILE"),
        4026 => Some("ANIMATION_TREE_NODE_WEIGHT"),
        4027 => Some("ANIMATION_TREE_NODE_FLAGS"),
        4028 => Some("ANIMATION_TREE_NODE_FIRST_CHILD"),
        4029 => Some("ANIMATION_TREE_NODE_NUM_CHILDREN"),
        4030 => Some("ANIMATION_TREE_NODE_PARENT"),
        4031 => Some("ANIMATION_BLENDCURVE_ANIMFROM"),
        4032 => Some("ANIMATION_BLENDCURVE_ANIMTO"),
        4033 => Some("ANIMATION_BLENDCURVE_DATA"),
        4034 => Some("ANIMATION_BLENDCURVE_LIST"),
        4035 => Some("ANIMATION_KEY_TIME"),
        4036 => Some("ANIMATION_KEY_DATA0"),
        4037 => Some("ANIMATION_KEY_DATA1"),
        4038 => Some("ANIMATION_KEY_DATA2"),
        4039 => Some("ANIMATION_KEY_DATA3"),
        4040 => Some("ANIMATION_IGNORESCALE"),
        5000 => Some("CUTSCENE_RUN_TIME"),
        5001 => Some("CUTSCENE_END_SCRIPT"),
        5002 => Some("CUTSCENE_LAYOUT"),
        5003 => Some("CUTSCENE_POSITION"),
        5004 => Some("CUTSCENE_ORIENTATION"),
        5005 => Some("CUTSCENE_TRANSITION_TIME"),
        5006 => Some("CUTSCENE_FOV"),
        5007 => Some("CUTSCENE_BLENDTREE"),
        5008 => Some("CUTSCENE_ANIMATIC"),
        5009 => Some("CUTSCENE_SHOWAREADYNAMICS"),
        5010 => Some("CUTSCENE_STAGED"),
        5011 => Some("CUTSCENE_LOD_CURVES"),
        5012 => Some("CUTSCENE_ANIM_SOUND_EVENTS"),
        5013 => Some("CUTSCENE_ENABLE_LEVEL_FBES"),
        5014 => Some("CUTSCENE_LOD_ORIGIN_POS"),
        5015 => Some("CUTSCENE_LOD_ORIGIN_ORI"),
        5016 => Some("CUTSCENE_FPS"),
        5017 => Some("CUTSCENE_STAGE_RESREF"),
        5018 => Some("CUTSCENE_PLAY_UNTIL_VO_COMPLETES"),
        5019 => Some("CUTSCENE_AREA_REQUIRED"),
        5020 => Some("CUTSCENE_SHADOW_RADIUS"),
        5021 => Some("CUTSCENE_LIGHT_OCCLUSION"),
        5050 => Some("CUTSCENE_HENCHMAN_TAG"),
        5051 => Some("CUTSCENE_HENCHMAN_ACTIONS"),
        5100 => Some("CUTSCENE_RESOURCES"),
        5101 => Some("CUTSCENE_RESOURCE_RESREF"),
        5102 => Some("CUTSCENE_RESOURCE_TYPE"),
        5200 => Some("CUTSCENE_ACTORS"),
        5201 => Some("CUTSCENE_ACTOR_ID"),
        5202 => Some("CUTSCENE_ACTOR_MODEL_RESREF"),
        5203 => Some("CUTSCENE_ACTOR_DEPRECATED_1"),
        5204 => Some("CUTSCENE_ACTOR_DEPRECATED_2"),
        5205 => Some("CUTSCENE_ACTOR_DEPRECATED_3"),
        5206 => Some("CUTSCENE_ACTOR_ACTION_QUEUE"),
        5207 => Some("CUTSCENE_ACTOR_DEPRECATED_4"),
        5208 => Some("CUTSCENE_ACTOR_CREATURE_RESREF"),
        5209 => Some("CUTSCENE_ACTOR_CAMERA_TARGET"),
        5210 => Some("CUTSCENE_ACTOR_USE_POSE"),
        5211 => Some("CUTSCENE_ACTOR_POSE"),
        5212 => Some("CUTSCENE_ACTOR_POSE_SPEED"),
        5213 => Some("CUTSCENE_ACTOR_POSE_HUMANOID"),
        5214 => Some("CUTSCENE_ACTOR_ORIGIN_POS"),
        5215 => Some("CUTSCENE_ACTOR_ORIGIN_ORI"),
        5216 => Some("CUTSCENE_ACTOR_MAPPING_TAG"),
        5217 => Some("CUTSCENE_ACTOR_INVENTORY"),
        5218 => Some("CUTSCENE_ACTOR_TRANSITION_DELAY"),
        5219 => Some("CUTSCENE_ACTOR_PREVIOUS_POSE"),
        5220 => Some("CUTSCENE_ACTOR_MAPPING_REQUIRED"),
        5221 => Some("CUTSCENE_ACTOR_FINAL_POS"),
        5222 => Some("CUTSCENE_ACTOR_FINAL_ORI"),
        5223 => Some("CUTSCENE_ACTOR_MASTER"),
        5224 => Some("CUTSCENE_ACTOR_LOD"),
        5225 => Some("CUTSCENE_ACTOR_AMBIENT_ANIM"),
        5226 => Some("CUTSCENE_ACTOR_MODEL_SCALE"),
        5300 => Some("CUTSCENE_ACTION_TYPE"),
        5301 => Some("CUTSCENE_ACTION_START_TIME"),
        5302 => Some("CUTSCENE_ACTION_STOP_TIME"),
        5303 => Some("CUTSCENE_ACTION_CURVES"),
        5304 => Some("CUTSCENE_ACTION_CATEGORY"),
        5350 => Some("CUTSCENE_ACTION_CURVE_BASE_VALUE"),
        5351 => Some("CUTSCENE_ACTION_CURVE_VERTICES"),
        5352 => Some("CUTSCENE_ACTION_CURVE_TRANSITIONS"),
        5353 => Some("CUTSCENE_ACTION_CURVE_DEPRECATED"),
        5370 => Some("CUTSCENE_ACTION_CURVE_VERTEX_TIME"),
        5371 => Some("CUTSCENE_ACTION_CURVE_VERTEX_VALUE"),
        5380 => Some("CUTSCENE_ACTION_CURVE_TRANSITION_TYPE"),
        5381 => Some("CUTSCENE_ACTION_CURVE_TRANSITION_CONTROL_1"),
        5382 => Some("CUTSCENE_ACTION_CURVE_TRANSITION_CONTROL_2"),
        5400 => Some("CUTSCENE_ACTION_ANIM_ANIMATION_NAME"),
        5401 => Some("CUTSCENE_ACTION_ANIM_BLENDTREE_NAME"),
        5402 => Some("CUTSCENE_ACTION_ANIM_SPEED"),
        5403 => Some("CUTSCENE_ACTION_ANIM_START_OFFSET"),
        5404 => Some("CUTSCENE_ACTION_ANIM_DEPRECATED1"),
        5405 => Some("CUTSCENE_ACTION_ANIM_PLAY_GAD"),
        5406 => Some("CUTSCENE_ACTION_ANIM_POSE_ANIMATION"),
        5407 => Some("CUTSCENE_ACTION_ANIM_LINK_TO_MOVEMENT"),
        5408 => Some("CUTSCENE_ACTION_ANIM_GAD_KEYS_POSITION"),
        5409 => Some("CUTSCENE_ACTION_ANIM_GAD_KEYS_ORIENTATION"),
        5410 => Some("CUTSCENE_ACTION_ANIM_BLEND_GAD"),
        5411 => Some("CUTSCENE_ACTION_ANIM_EXTEND_GAD"),
        5412 => Some("CUTSCENE_ACTION_ANIM_LINK_TO_MOVEMENT_DISTANCES"),
        5413 => Some("CUTSCENE_ACTION_ANIM_APPLY_TO_FUTURE_GADS"),
        5520 => Some("CUTSCENE_ACTION_FRAME_BUFFER_EFFECT_FILE_NAME"),
        5521 => Some("CUTSCENE_ACTION_FRAME_BUFFER_EFFECT_EFFECT_NAME"),
        5522 => Some("CUTSCENE_ACTION_FRAME_BUFFER_EFFECT_PARAM_LIST"),
        5523 => Some("CUTSCENE_ACTION_FRAME_BUFFER_EFFECT_PARAM_NAME"),
        5524 => Some("CUTSCENE_ACTION_FRAME_BUFFER_EFFECT_PARAM_VALUE"),
        5525 => Some("CUTSCENE_ACTION_FRAME_BUFFER_EFFECT_PARAM_CURVE_INDEX"),
        5526 => Some("CUTSCENE_ACTION_FRAME_BUFFER_EFFECT_PARAM_VECTOR_INDEX"),
        5562 => Some("CUTSCENE_ACTION_SPEAK_LINE_LIPSYNCH_SET"),
        5563 => Some("CUTSCENE_ACTION_SPEAK_LINE_VOBANK"),
        5564 => Some("CUTSCENE_ACTION_SPEAK_LINE_FAHEADMOVEMENT"),
        5565 => Some("CUTSCENE_ACTION_SPEAK_LINE_NOVOINGAME"),
        5570 => Some("CUTSCENE_ACTION_STAGE_CAMERA_DEFAULT_CAMERA"),
        5571 => Some("CUTSCENE_ACTION_STAGE_CAMERA_HENCHMAN_CAMERA"),
        5580 => Some("CUTSCENE_ACTION_STAGE_PLACE_LOOK_AT"),
        5600 => Some("CUTSCENE_ACTION_SHAKE_TYPE"),
        5601 => Some("CUTSCENE_ACTION_SHAKE_DEPRECATED1"),
        5602 => Some("CUTSCENE_ACTION_SHAKE_DEPRECATED2"),
        5603 => Some("CUTSCENE_ACTION_SHAKE_NOISE_SEED"),
        5604 => Some("CUTSCENE_ACTION_SHAKE_NOISE_FREQUENCY"),
        5605 => Some("CUTSCENE_ACTION_SHAKE_NOISE_TYPE"),
        5606 => Some("CUTSCENE_ACTION_SHAKE_NOISE_CORRELATED"),
        5607 => Some("CUTSCENE_ACTION_SHAKE_NOISE_ROUGHNESS"),
        5608 => Some("CUTSCENE_ACTION_SHAKE_NOISE_RAMP_IN"),
        5609 => Some("CUTSCENE_ACTION_SHAKE_NOISE_RAMP_OUT"),
        5610 => Some("CUTSCENE_ACTION_ACTIVE_CAMERA_ACTOR_ID"),
        5620 => Some("CUTSCENE_ACTION_HEADTRACKING_TARGET_ID"),
        5621 => Some("CUTSCENE_ACTION_HEADTRACKING_SPEED"),
        5624 => Some("CUTSCENE_ACTION_HEADTRACKING_DEPRECATED1"),
        5625 => Some("CUTSCENE_ACTION_HEADTRACKING_DEPRECATED2"),
        5626 => Some("CUTSCENE_ACTION_HEADTRACKING_DEPRECATED3"),
        5627 => Some("CUTSCENE_ACTION_HEADTRACKING_DEPRECATED4"),
        5628 => Some("CUTSCENE_ACTION_HEADTRACKING_REALIGN_START"),
        5629 => Some("CUTSCENE_ACTION_HEADTRACKING_REALIGN_CONT"),
        5630 => Some("CUTSCENE_ACTION_LINK_ACTOR_TARGET_ID"),
        5631 => Some("CUTSCENE_ACTION_LINK_ACTOR_NODE_ID"),
        5632 => Some("CUTSCENE_ACTION_LINK_ACTOR_DEPRECATED1"),
        5633 => Some("CUTSCENE_ACTION_LINK_ACTOR_DEPRECATED2"),
        5634 => Some("CUTSCENE_ACTION_LINK_ACTOR_DEPRECATED3"),
        5635 => Some("CUTSCENE_ACTION_LINK_ACTOR_DEPRECATED4"),
        5636 => Some("CUTSCENE_ACTION_LINK_ACTOR_IS_TARGET_CRUST"),
        5637 => Some("CUTSCENE_ACTION_LINK_ACTOR_USE_OFFSET"),
        5640 => Some("CUTSCENE_ACTION_APPLYCRUST_TARGET_ID"),
        5650 => Some("CUTSCENE_ACTION_POSE_ANIMATION_POSE"),
        5651 => Some("CUTSCENE_ACTION_POSE_ANIMATION_ANIMATION"),
        5652 => Some("CUTSCENE_ACTION_POSE_ANIMATION_LOOPING"),
        5653 => Some("CUTSCENE_ACTION_POSE_ANIMATION_OUTRO"),
        5654 => Some("CUTSCENE_ACTION_POSE_ANIMATION_OUTRO_SPEED"),
        5670 => Some("CUTSCENE_ACTION_SOUND_NAME"),
        5671 => Some("CUTSCENE_ACTION_SOUND_SPLINE_PARAM_NO1"),
        5672 => Some("CUTSCENE_ACTION_SOUND_SPLINE_PARAM_NO2"),
        5673 => Some("CUTSCENE_ACTION_SOUND_SPLINE_PARAM_NO3"),
        5674 => Some("CUTSCENE_ACTION_SOUND_SPLINE_PARAM_NO4"),
        5675 => Some("CUTSCENE_ACTION_SOUND_SPLINE_PARAM_NO5"),
        5680 => Some("CUTSCENE_ACTION_CHANGEVISIBILITY_VISIBLE"),
        5700 => Some("CUTSCENE_ACTION_TOGGLE_CLOTH_PHYSICS"),
        5701 => Some("CUTSCENE_ACTION_TOGGLE_HAIR_PHYSICS"),
        5720 => Some("CUTSCENE_ACTION_SET_LOD_DEPRECATED"),
        5730 => Some("CUTSCENE_ACTION_DRAW_WEAPON_MAIN"),
        5731 => Some("CUTSCENE_ACTION_DRAW_WEAPON_OFF"),
        5740 => Some("CUTSCENE_ACTION_PLAYMOVIE"),
        5750 => Some("CUTSCENE_ACTION_SETGORE"),
        6000 => Some("MMH_NAME"),
        6001 => Some("MMH_MATERIAL_OBJECT"),
        6002 => Some("MMH_MATERIAL_LIBRARY"),
        6003 => Some("MMH_RESNAME"),
        6004 => Some("MMH_ID"),
        6005 => Some("MMH_MODEL_HIERARCHY_MODEL_DATA_NAME"),
        6006 => Some("MMH_MESH_GROUP_NAME"),
        6007 => Some("MMH_NODE_POINT_LIGHT_COLOR"),
        6008 => Some("MMH_NODE_POINT_LIGHT_RADIUS"),
        6009 => Some("MMH_NODE_POINT_LIGHT_IS_STATIC"),
        6010 => Some("MMH_NODE_AMBIENT_LIGHT_COLOR"),
        6011 => Some("MMH_NODE_EMITTER_BIRTH_RATE"),
        6012 => Some("MMH_NODE_EMITTER_BIRTH_RATE_RANGE"),
        6013 => Some("MMH_NODE_EMITTER_LIFE"),
        6014 => Some("MMH_NODE_EMITTER_LIFE_RANGE"),
        6015 => Some("MMH_NODE_EMITTER_SCALE_RANGE"),
        6016 => Some("MMH_NODE_EMITTER_INITIAL_SPEED"),
        6017 => Some("MMH_NODE_EMITTER_INITIAL_SPEED_RANGE"),
        6018 => Some("MMH_NODE_EMITTER_ACCELERATION"),
        6019 => Some("MMH_NODE_EMITTER_INITIAL_ROTATION_SPEED"),
        6020 => Some("MMH_NODE_EMITTER_INITIAL_ROTATION_SPEED_RANGE"),
        6021 => Some("MMH_NODE_EMITTER_ROTATIONAL_ACCELERATION"),
        6022 => Some("MMH_NODE_INV_EMITTER_MOVEMENT_SPREAD_UPDATE_DELAY"),
        6023 => Some("MMH_NODE_EMITTER_SPAWN_SPREAD_X"),
        6024 => Some("MMH_NODE_EMITTER_SPAWN_SPREAD_Y"),
        6025 => Some("MMH_NODE_EMITTER_MOVEMENT_SPREAD_X"),
        6026 => Some("MMH_NODE_EMITTER_MOVEMENT_SPREAD_Y"),
        6027 => Some("MMH_NODE_EMITTER_OPTIONS_BITFLAGS"),
        6028 => Some("MMH_NODE_EMITTER_OPTIONS_BIRTHRATE_IN_PARTICLES_PER_METER"),
        6029 => Some("MMH_NODE_EMITTER_OPTIONS_RANDOM_INITIAL_ROTATION"),
        6030 => Some("MMH_NODE_EMITTER_OPTIONS_PARTICLES_AFFECTED_BY_WIND"),
        6031 => Some("MMH_NODE_EMITTER_GRAVITY_MULTIPLIER"),
        6032 => Some("MMH_NODE_EMITTER_OPTIONS_PARTICLES_FOLLOW_PATH"),
        6033 => Some("MMH_NODE_EMITTER_OPTIONS_LINK_PARTICLES_TOGETHER"),
        6034 => Some("MMH_NODE_EMITTER_OPTIONS_UPDATE_ONLY_WHEN_VISIBLE"),
        6035 => Some("MMH_NODE_EMITTER_OPTIONS_ENABLE_PARTICLE_COLLISIONS"),
        6036 => Some("MMH_NODE_EMITTER_OPTIONS_INHERIT_VELOCITY_INSTEAD_OF_POSITION"),
        6037 => Some("MMH_NODE_EMITTER_ORIENTATION_BEHAVIOR"),
        6038 => Some("MMH_NODE_EMITTER_PARTICLE_INHERITANCE"),
        6039 => Some("MMH_NODE_AGE_MAP_COUNT"),
        6040 => Some("MMH_NODE_AGE_MAP_ELEMENT_PERCENT_LIFE_ELAPSED"),
        6041 => Some("MMH_NODE_AGE_MAP_ELEMENT_SCALE_X"),
        6042 => Some("MMH_NODE_AGE_MAP_ELEMENT_SCALE_Y"),
        6043 => Some("MMH_NODE_AGE_MAP_ELEMENT_COLOR"),
        6044 => Some("MMH_NODE_SPAWN_VOLUME_OPTIONS_BITFLAGS"),
        6045 => Some("MMH_NODE_SPAWN_VOLUME_OPTIONS_SPAWN_WITHIN_VOLUME"),
        6046 => Some("MMH_NODE_SPAWN_VOLUME_OPTIONS_INVERT_SPAWN_VOLUME_NORMALS"),
        6047 => Some("MMH_TRANSLATION"),
        6048 => Some("MMH_ROTATION"),
        6049 => Some("MMH_ATTRIBUTE_NAME"),
        6050 => Some("MMH_ATTRIBUTE_SOURCE_NAME"),
        6051 => Some("MMH_EXPORT_TAG_NAME"),
        6052 => Some("MMH_EXPORT_EXPORT_NAME"),
        6053 => Some("MMH_EXPORT_CONTROLLER_TYPE"),
        6054 => Some("MMH_BOUNDING_BOX_MIN"),
        6055 => Some("MMH_BOUNDING_BOX_MAX"),
        6056 => Some("MMH_NODE_COLLISION_OBJ_DENSITY"),
        6057 => Some("MMH_NODE_COLLISION_OBJ_TYPE"),
        6058 => Some("MMH_SHAPE_TYPE"),
        6059 => Some("MMH_SHAPE_PMAT_NAME"),
        6060 => Some("MMH_SHAPE_ROTATION"),
        6061 => Some("MMH_SHAPE_POSITION"),
        6062 => Some("MMH_SHAPE_COLLISION_MASK_BITFLAGS"),
        6063 => Some("MMH_SHAPE_COLLISION_MASK_ALL"),
        6064 => Some("MMH_SHAPE_COLLISION_MASK_NONE"),
        6065 => Some("MMH_SHAPE_COLLISION_MASK_ITEMS"),
        6066 => Some("MMH_SHAPE_COLLISION_MASK_CREATURES"),
        6067 => Some("MMH_SHAPE_COLLISION_MASK_PLACEABLES"),
        6068 => Some("MMH_SHAPE_COLLISION_MASK_TRIGGERS"),
        6069 => Some("MMH_SHAPE_COLLISION_MASK_STATIC_GEOMETRY"),
        6070 => Some("MMH_SHAPE_COLLISION_MASK_NONWALKABLE"),
        6071 => Some("MMH_SHAPE_BOX_DIM"),
        6072 => Some("MMH_SHAPE_RADIUS"),
        6073 => Some("MMH_SHAPE_HEIGHT"),
        6074 => Some("MMH_SHAPE_MESH_SHAPE_FLAGS"),
        6075 => Some("MMH_SHAPE_MESH_HEIGHT_FIELD_AXIS"),
        6076 => Some("MMH_SHAPE_MESH_HEIGHT_FIELD_EXTENT"),
        6077 => Some("MMH_SHAPE_COOKED_DATA_STREAM"),
        6078 => Some("MMH_JOINT_TARGET"),
        6079 => Some("MMH_JOINT_TYPE"),
        6080 => Some("MMH_JOINT_TYPE_STRUCT"),
        6081 => Some("MMH_JOINT_LOCAL_NORMAL_1"),
        6082 => Some("MMH_JOINT_LOCAL_NORMAL_2"),
        6083 => Some("MMH_JOINT_LOCAL_ANCHOR_1"),
        6084 => Some("MMH_JOINT_LOCAL_ANCHOR_2"),
        6085 => Some("MMH_JOINT_LOCAL_AXIS_1"),
        6086 => Some("MMH_JOINT_LOCAL_AXIS_2"),
        6087 => Some("MMH_JOINT_MAX_FORCE"),
        6088 => Some("MMH_JOINT_MAX_TORQUE"),
        6089 => Some("MMH_JOINT_COLLISION_ENABLED"),
        6090 => Some("MMH_JOINT_SPHERICAL_SWING_AXIS"),
        6091 => Some("MMH_JOINT_SPHERICAL_PROJECTION_DISTANCE"),
        6092 => Some("MMH_JOINT_SPHERICAL_TWIST_LIMIT_LOW"),
        6093 => Some("MMH_JOINT_SPHERICAL_TWIST_LIMIT_HIGH"),
        6094 => Some("MMH_JOINT_SPHERICAL_SWING_LIMIT"),
        6095 => Some("MMH_JOINT_SPHERICAL_TWIST_SWING"),
        6096 => Some("MMH_JOINT_SPHERICAL_SWING_SPRING"),
        6097 => Some("MMH_JOINT_SPHERICAL_JOINT_SPRING"),
        6098 => Some("MMH_JOINT_SPHERICAL_PROJECTION_MODE"),
        6099 => Some("MMH_JOINT_SPHERICAL_SPHERE_FLAGS"),
        6100 => Some("MMH_JOINT_REVOLUTE_LIMIT_LOW"),
        6101 => Some("MMH_JOINT_REVOLUTE_LIMIT_HIGH"),
        6102 => Some("MMH_JOINT_REVOLUTE_PROJECTION_DISTANCE"),
        6103 => Some("MMH_JOINT_REVOLUTE_PROJECTION_ANGLE"),
        6104 => Some("MMH_JOINT_REVOLUTE_PROJECTION_MODE"),
        6105 => Some("MMH_JOINT_REVOLUTE_SPRING"),
        6106 => Some("MMH_JOINT_REVOLUTE_MOTOR_VEL_TARGET"),
        6107 => Some("MMH_JOINT_REVOLUTE_MOTOR_MAX_FORCE"),
        6108 => Some("MMH_JOINT_REVOLUTE_MOTOR_FREE_SPIN"),
        6109 => Some("MMH_JOINT_REVOLUTE_REVOLUTE_FLAGS"),
        6110 => Some("MMH_JOINT_DISTANCE_MIN_DISTANCE"),
        6111 => Some("MMH_JOINT_DISTANCE_MAX_DISTANCE"),
        6112 => Some("MMH_JOINT_DISTANCE_SPRING"),
        6113 => Some("MMH_JOINT_DISTANCE_DISTANCE_FLAGS"),
        6114 => Some("MMH_JOINT_PULLEY_PULLEY_1"),
        6115 => Some("MMH_JOINT_PULLEY_PULLEY_2"),
        6116 => Some("MMH_JOINT_PULLEY_DISTANCE"),
        6117 => Some("MMH_JOINT_PULLEY_STIFFNESS"),
        6118 => Some("MMH_JOINT_PULLEY_RATIO"),
        6119 => Some("MMH_JOINT_PULLEY_MOTOR_VEL_TARGET"),
        6120 => Some("MMH_JOINT_PULLEY_MOTOR_MAX_FORCE"),
        6121 => Some("MMH_JOINT_PULLEY_MOTOR_FREE_SPIN"),
        6122 => Some("MMH_JOINT_PULLEY_PULLEY_FLAGS"),
        6123 => Some("MMH_JOINT_6DOF_X_MOTION"),
        6124 => Some("MMH_JOINT_6DOF_Y_MOTION"),
        6125 => Some("MMH_JOINT_6DOF_Z_MOTION"),
        6126 => Some("MMH_JOINT_6DOF_SWING_1_MOTION"),
        6127 => Some("MMH_JOINT_6DOF_SWING_2_MOTION"),
        6128 => Some("MMH_JOINT_6DOF_TWIST_MOTION"),
        6129 => Some("MMH_JOINT_6DOF_LINEAR_LIMIT"),
        6130 => Some("MMH_JOINT_6DOF_SWING_1_LIMIT"),
        6131 => Some("MMH_JOINT_6DOF_SWING_2_LIMIT"),
        6132 => Some("MMH_JOINT_6DOF_TWIST_LIMIT_LOW"),
        6133 => Some("MMH_JOINT_6DOF_TWIST_LIMIT_HIGH"),
        6134 => Some("MMH_JOINT_6DOF_DRIVE_ORIENTATION"),
        6135 => Some("MMH_JOINT_6DOF_DRIVE_X_DRIVE_TYPE"),
        6136 => Some("MMH_JOINT_6DOF_DRIVE_X_DRIVE_SPRING"),
        6137 => Some("MMH_JOINT_6DOF_DRIVE_X_DRIVE_DAMPING"),
        6138 => Some("MMH_JOINT_6DOF_DRIVE_X_DRIVE_FORCE_LIMIT"),
        6139 => Some("MMH_JOINT_6DOF_DRIVE_Y_DRIVE_TYPE"),
        6140 => Some("MMH_JOINT_6DOF_DRIVE_Y_DRIVE_SPRING"),
        6141 => Some("MMH_JOINT_6DOF_DRIVE_Y_DRIVE_DAMPING"),
        6142 => Some("MMH_JOINT_6DOF_DRIVE_Y_DRIVE_FORCE_LIMIT"),
        6143 => Some("MMH_JOINT_6DOF_DRIVE_Z_DRIVE_TYPE"),
        6144 => Some("MMH_JOINT_6DOF_DRIVE_Z_DRIVE_SPRING"),
        6145 => Some("MMH_JOINT_6DOF_DRIVE_Z_DRIVE_DAMPING"),
        6146 => Some("MMH_JOINT_6DOF_DRIVE_Z_DRIVE_FORCE_LIMIT"),
        6147 => Some("MMH_JOINT_6DOF_DRIVE_SWING_DRIVE_TYPE"),
        6148 => Some("MMH_JOINT_6DOF_DRIVE_SWING_DRIVE_SPRING"),
        6149 => Some("MMH_JOINT_6DOF_DRIVE_SWING_DRIVE_DAMPING"),
        6150 => Some("MMH_JOINT_6DOF_DRIVE_SWING_DRIVE_FORCE_LIMIT"),
        6151 => Some("MMH_JOINT_6DOF_DRIVE_TWIST_DRIVE_TYPE"),
        6152 => Some("MMH_JOINT_6DOF_DRIVE_TWIST_DRIVE_SPRING"),
        6153 => Some("MMH_JOINT_6DOF_DRIVE_TWIST_DRIVE_DAMPING"),
        6154 => Some("MMH_JOINT_6DOF_DRIVE_TWIST_DRIVE_FORCE_LIMIT"),
        6155 => Some("MMH_JOINT_6DOF_DRIVE_SLERP_DRIVE_TYPE"),
        6156 => Some("MMH_JOINT_6DOF_DRIVE_SLERP_DRIVE_SPRING"),
        6157 => Some("MMH_JOINT_6DOF_DRIVE_SLERP_DRIVE_DAMPING"),
        6158 => Some("MMH_JOINT_6DOF_DRIVE_SLERP_DRIVE_FORCE_LIMIT"),
        6159 => Some("MMH_JOINT_6DOF_DRIVE_POSITION"),
        6160 => Some("MMH_JOINT_6DOF_DRIVE_LINEAR_VELOCITY"),
        6161 => Some("MMH_JOINT_6DOF_DRIVE_ANGULAR_VELOCITY"),
        6162 => Some("MMH_JOINT_6DOF_PROJECTION_DISTANCE"),
        6163 => Some("MMH_JOINT_6DOF_PROJECTION_ANGLE"),
        6164 => Some("MMH_JOINT_6DOF_GEAR_RATIO"),
        6165 => Some("MMH_JOINT_6DOF_PROJECTION_MODE"),
        6166 => Some("MMH_JOINT_6DOF_D6_FLAGS"),
        6167 => Some("MMH_DATA_SEMANTIC"),
        6168 => Some("MMH_DATA_IS_INDEX_STREAM"),
        6169 => Some("MMH_DATA_TYPE"),
        6170 => Some("MMH_DATA_BITFLAGS"),
        6171 => Some("MMH_DATA_LOOPING"),
        6172 => Some("MMH_DATA_INSTANCED"),
        6173 => Some("MMH_DATA_COUNT"),
        6174 => Some("MMH_DATA_PRIMITIVE_TYPE"),
        6175 => Some("MMH_DATA_FREQUENCY"),
        6176 => Some("MMH_MESH_CAST_RUNTIME_SHADOW"),
        6177 => Some("MMH_MESH_CAST_BAKED_SHADOW"),
        6178 => Some("MMH_SHAPE_COLLISION_MASK_EFFECTS"),
        6179 => Some("MMH_SHAPE_COLLISION_MASK_WAYPOINTS"),
        6180 => Some("MMH_FLIPBOOK_FRAMES_PER_SECOND"),
        6181 => Some("MMH_FLIPBOOK_ROWS"),
        6182 => Some("MMH_FLIPBOOK_COLUMNS"),
        6183 => Some("MMH_FLIPBOOK_RANDOM_START_FRAME"),
        6184 => Some("MMH_EMITTER_TARGET_NAME"),
        6185 => Some("MMH_EMITTER_TARGET_ATTRACTION"),
        6186 => Some("MMH_EMITTER_TARGET_RADIUS"),
        6187 => Some("MMH_EMITTER_SPAWN_DIRECTION_TRACKS_TARGET"),
        6188 => Some("MMH_EMITTER_KILL_PARTICLE_WHEN_TARGET_HIT"),
        6189 => Some("MMH_EMITTER_FLIPBOOK_TYPE"),
        6190 => Some("MMH_LIGHTPROBE_IRRADIANCE_COEFFICIENTS_RED"),
        6191 => Some("MMH_LIGHTPROBE_IRRADIANCE_COEFFICIENTS_GREEN"),
        6192 => Some("MMH_LIGHTPROBE_IRRADIANCE_COEFFICIENTS_BLUE"),
        6193 => Some("MMH_MESH_CUT_AWAY"),
        6194 => Some("MMH_MESH_PUNCH_THROUGH"),
        6195 => Some("MMH_CLOTH_THICKNESS"),
        6196 => Some("MMH_CLOTH_DENSITY"),
        6197 => Some("MMH_CLOTH_BENDING_STIFFNESS"),
        6198 => Some("MMH_CLOTH_STRETCHING_STIFFNESS"),
        6199 => Some("MMH_CLOTH_DAMPING_COEFFICIENT"),
        6200 => Some("MMH_CLOTH_FRICTION"),
        6201 => Some("MMH_CLOTH_PRESSURE"),
        6202 => Some("MMH_CLOTH_TEAR_FACTOR"),
        6203 => Some("MMH_CLOTH_COLLISION_RESPONSE_COEFFICIENT"),
        6204 => Some("MMH_CLOTH_ATTACHMENT_RESPONSE_COEFFICIENT"),
        6205 => Some("MMH_CLOTH_ATTACHMENT_TEAR_FACTOR"),
        6206 => Some("MMH_CLOTH_SOLVER_ITERATIONS"),
        6207 => Some("MMH_CLOTH_EXTERNAL_ACCELERATION"),
        6208 => Some("MMH_CLOTH_WAKE_UP_COUNTER"),
        6209 => Some("MMH_CLOTH_SLEEP_LINEAR_VELOCITY"),
        6210 => Some("MMH_CLOTH_FLAG_BITFLAGS"),
        6211 => Some("MMH_CLOTH_FLAG_PRESSURE"),
        6212 => Some("MMH_CLOTH_FLAG_STATIC"),
        6213 => Some("MMH_CLOTH_FLAG_DISABLE_COLLISION"),
        6214 => Some("MMH_CLOTH_FLAG_SELFCOLLISION"),
        6215 => Some("MMH_CLOTH_FLAG_VISUALIZATION"),
        6216 => Some("MMH_CLOTH_FLAG_GRAVITY"),
        6217 => Some("MMH_CLOTH_FLAG_BENDING"),
        6218 => Some("MMH_CLOTH_FLAG_BENDING_ORTHO"),
        6219 => Some("MMH_CLOTH_FLAG_DAMPING"),
        6220 => Some("MMH_CLOTH_FLAG_COLLISION_TWOWAY"),
        6221 => Some("MMH_CLOTH_FLAG_TRIANGLE_COLLISION"),
        6222 => Some("MMH_CLOTH_FLAG_TEARABLE"),
        6223 => Some("MMH_CLOTH_FLAG_HARDWARE"),
        6224 => Some("MMH_CLOTH_FLAG_COMDAMPING"),
        6225 => Some("MMH_CLOTH_ATTACHMENT_TYPE"),
        6226 => Some("MMH_CLOTH_ATTACHMENT_FLAG_BITFLAGS"),
        6227 => Some("MMH_CLOTH_ATTACHMENT_FLAG_TWO_WAY_ATTACHMENT"),
        6228 => Some("MMH_CLOTH_ATTACHMENT_FLAG_TEARABLE_ATTACHMENT"),
        6229 => Some("MMH_CLOTH_ATTACHMENT_SHAPE_NAME"),
        6230 => Some("MMH_CLOTH_ATTACHMENT_VERTEX_ID"),
        6231 => Some("MMH_CLOTH_ATTACHMENT_LOCAL_POS"),
        6232 => Some("MMH_CLOTH_COOKED_DATA_STREAM"),
        6233 => Some("MMH_CLOTH_MESH_GROUP_STRUCT"),
        6234 => Some("MMH_NODE_EMITTER_TYPE"),
        6235 => Some("MMH_NODE_CRUST_HOOK_ID"),
        6236 => Some("MMH_COLLISION_OBJECT_VOLUME"),
        6237 => Some("MMH_OBJECT_VOLUME"),
        6238 => Some("MMH_EXPORT_TAG_VARIABLE_TYPE"),
        6239 => Some("MMH_EMITTER_IS_PHYSICS_EMITTER"),
        6240 => Some("MMH_SHAPE_VOLUME"),
        6241 => Some("MMH_SHAPE_NAME"),
        6242 => Some("MMH_SNAP_POSITION"),
        6243 => Some("MMH_EMITTER_IS_PHYSICS_OBJECT_SPAWN_EMITTER"),
        6244 => Some("MMH_SHAPE_ALLOW_EMITTER_SPAWN"),
        6245 => Some("MMH_COLLISION_GROUP"),
        6246 => Some("MMH_EMITTER_EMITTER_ATTACHMENT_TYPE"),
        6247 => Some("MMH_EMITTER_EMITTER_ATTACHMENT_NAME"),
        6248 => Some("MMH_FACIAL_ANIMATION_BLUEPRINT_NAME"),
        6249 => Some("MMH_NODE_POINT_LIGHT_INTENSITY_VARIATION"),
        6250 => Some("MMH_NODE_POINT_LIGHT_INTENSITY_PERIOD"),
        6251 => Some("MMH_NODE_POINT_LIGHT_INTENSITY_PERIOD_DELTA"),
        6252 => Some("MMH_SHAPE_FADEABLE"),
        6253 => Some("MMH_LIGHTPROBE_IRRADIANCE_RES"),
        6254 => Some("MMH_BONE_INDEX"),
        6255 => Some("MMH_MESH_BONES_USED"),
        6256 => Some("MMH_TOTAL_BONES"),
        6257 => Some("MMH_CLOTH_WIND_ENABLED"),
        6258 => Some("MMH_CLOTH_WIND_SPACE"),
        6259 => Some("MMH_CLOTH_WIND_DIRECTION"),
        6260 => Some("MMH_CLOTH_WIND_RESPONSE"),
        6261 => Some("MMH_CLOTH_WIND_RESPONSE_LIMIT"),
        6262 => Some("MMH_CLOTH_WIND_STRENGTH"),
        6263 => Some("MMH_CLOTH_WIND_GUST_MIN_STRENGTH"),
        6264 => Some("MMH_CLOTH_WIND_GUST_MAX_STRENGTH"),
        6265 => Some("MMH_CLOTH_WIND_GUST_MIN_DURATION"),
        6266 => Some("MMH_CLOTH_WIND_GUST_MAX_DURATION"),
        6267 => Some("MMH_CLOTH_WIND_GUST_MIN_INTERVAL"),
        6268 => Some("MMH_CLOTH_WIND_GUST_MAX_INTERVAL"),
        6269 => Some("MMH_CLOTH_WIND_GUST_DIR_CHANGE"),
        6270 => Some("MMH_CLOTH_WIND_GUST_AXIS_RATIO"),
        6271 => Some("MMH_CLOTH_WIND_SPEEDTREE_UPDATE_TIME"),
        6272 => Some("MMH_CLOTH_WIND_SPEEDTREE_STRENGTH"),
        6273 => Some("MMH_CLOTH_WIND_SPEEDTREE_DIRECTION"),
        6274 => Some("MMH_EXPORT_CONTROLLER_INDEX"),
        6275 => Some("MMH_TOTAL_EXPORTS"),
        6276 => Some("MMH_CLOTH_WIND_SPEEDTREE_PARAMS"),
        6277 => Some("MMH_SHAPE_COLLISION_MASK_WATER"),
        6278 => Some("MMH_SCALE"),
        6279 => Some("MMH_NODE_EMITTER_AGEMAP_COLOR_MULTIPLIER"),
        6280 => Some("MMH_NODE_EMITTER_AGEMAP_SCALEX_MULTIPLIER"),
        6281 => Some("MMH_NODE_EMITTER_AGEMAP_SCALEY_MULTIPLIER"),
        6282 => Some("MMH_NODE_EMITTER_OPTIONS_BOUNCINESS"),
        6283 => Some("MMH_NODE_EMITTER_OPTIONS_FRICTION"),
        6284 => Some("MMH_NODE_EMITTER_MESH_PARTICLE_MODELNAME"),
        6285 => Some("MMH_NODE_SPAWN_VOLUME_TYPE"),
        6286 => Some("MMH_NODE_SPAWN_VOLUME_RADIUS"),
        6287 => Some("MMH_NODE_SPAWN_VOLUME_CYLINDER_LENGTH"),
        6288 => Some("MMH_NODE_SPAWN_VOLUME_CYLINDER_AXIS"),
        6289 => Some("MMH_NODE_SPAWN_VOLUME_BOX_MIN"),
        6290 => Some("MMH_NODE_SPAWN_VOLUME_BOX_MAX"),
        6291 => Some("MMH_NODE_SPAWN_VOLUME_OPTIONS_NORMALS_AS_DIRECTION"),
        6292 => Some("MMH_WEAPONTRAIL_SEGMENT_LENGTH"),
        6293 => Some("MMH_WEAPONTRAIL_DURATION"),
        6294 => Some("MMH_NODE_EMITTER_WORLD_AXIS_ACCELERATION"),
        6295 => Some("MMH_SHAPE_COLLISION_MASK_TERRAIN_WALL"),
        6296 => Some("MMH_NODE_LIGHT_AFFECT_DOMAIN"),
        6297 => Some("MMN_NODE_EMITTER_VERTEX_FORMAT"),
        6298 => Some("MMH_NODE_EMITTER_OPTIONS_OBJECT_SPACE_ACCELERATION"),
        6299 => Some("MMH_NODE_EMITTER_INITIAL_ROTATION"),
        6300 => Some("MMH_NODE_EMITTER_INITIAL_ROTATION_RANGE"),
        6301 => Some("MMH_MESH_RECEIVE_BAKED_SHADOW"),
        6302 => Some("MMH_NODE_EMITTER_MESH_PARTICLE_UP_AXIS"),
        6303 => Some("MMH_NODE_EMITTER_MESH_PARTICLE_ROLL_AXIS"),
        6304 => Some("MMH_MESH_RECEIVE_RUNTIME_SHADOW"),
        6305 => Some("MMH_SHAPE_COLLISION_MASK_WALKABLE"),
        6306 => Some("MMH_MODEL_MESH_NAME_LIST"),
        6307 => Some("MMH_NODE_MESH_NAME"),
        6308 => Some("MMH_NODE_EMITTER_UV_DISTRIBUTION_SIZE"),
        6309 => Some("MMH_NODE_EMITTER_IGNORE_DISTORTION"),
        6310 => Some("MMH_NODE_EMITTER_SPLATPARAMS_WIDTH"),
        6311 => Some("MMH_NODE_EMITTER_SPLATPARAMS_HEIGHT"),
        6312 => Some("MMH_NODE_EMITTER_SPLATPARAMS_NUMSAMPLES_WIDTH"),
        6313 => Some("MMH_NODE_EMITTER_SPLATPARAMS_NUMSAMPLES_HEIGHT"),
        6314 => Some("MMH_NODE_EMITTER_SPLATPARAMS_ORIENTATION_RANGE"),
        6315 => Some("MMH_NODE_EMITTER_SPLATPARAMS_LIFE"),
        6316 => Some("MMH_NODE_EMITTER_SPLATPARAMS_FLIPBOOK_TYPE"),
        6317 => Some("MMH_NODE_EMITTER_SPLATPARAMS_FLIPBOOK_FRAMES_PER_SECOND"),
        6318 => Some("MMH_NODE_EMITTER_SPLATPARAMS_FLIPBOOK_ROWS"),
        6319 => Some("MMH_NODE_EMITTER_SPLATPARAMS_FLIPBOOK_COLUMNS"),
        6320 => Some("MMH_NODE_EMITTER_SPLATPARAMS_FLIPBOOK_RANDOM_START_FRAME"),
        6321 => Some("MMH_NODE_EMITTER_CAN_PARTICLES_SPLAT"),
        6322 => Some("MMH_NODE_EMITTER_SPLATPARAMS_AGE_MAP_ELEMENT_PERCENT_LIFE_ELAPSED"),
        6323 => Some("MMH_NODE_EMITTER_LOD"),
        6324 => Some("MMH_NODE_EMITTER_SPLATPARAMS_MATERIALNAME"),
        6325 => Some("MMH_NODE_EMITTER_USER_PARAM_NAME"),
        6327 => Some("MMH_REMOTE_MATERIAL_DECAL_NAME"),
        6328 => Some("MMH_REMOTE_MATERIAL_FRESNEL_FALLOFF"),
        6329 => Some("MMH_REMOTE_MATERIAL_INVERT_FRESNEL"),
        6330 => Some("MMH_NODE_SOUND_MATERIAL"),
        6331 => Some("MMH_REMOTE_MATERIAL_ALPHA"),
        6332 => Some("MMH_REMOTE_MATERIAL_TINT"),
        6333 => Some("MMH_EMITTER_PRESIMULATE_TIME"),
        6334 => Some("MMH_MESH_IS_VFX_MESH"),
        6335 => Some("MMH_MESH_MATERIAL_COLOR"),
        6336 => Some("MMH_LIGHTPROBE_IRRADIANCE_RED"),
        6337 => Some("MMH_LIGHTPROBE_IRRADIANCE_GREEN"),
        6338 => Some("MMH_LIGHTPROBE_IRRADIANCE_BLUE"),
        6339 => Some("MMH_LIGHT_CAN_BE_OCCLUDED"),
        6340 => Some("MMH_USE_VARIATION_TINT"),
        6341 => Some("MMH_NODE_EMITTER_SPLATPARAMS_HOLD_LAST_FRAME"),
        6342 => Some("MMH_EMITTER_EMITTER_ATTACHMENT_SPAWN_ON_SURFACE"),
        6343 => Some("MMH_EMITTER_EMITTER_ATTACHMENT_USE_NORMAL_FOR_VELOCITY"),
        6344 => Some("MMH_NODE_EMITTER_SPLATPARAMS_AGEMAP_COLOR_MULTIPLIER"),
        6345 => Some("MMH_NODE_LIGHT_VERSION"),
        6346 => Some("MMH_MESH_DEFAULT_HIDDEN"),
        6998 => Some("MMH_SHAPE_TYPE_STRUCT"),
        6999 => Some("MMH_CHILDREN"),
        7000 => Some("TERRAIN_VERSION"),
        7001 => Some("TERRAIN_BASE_ROWS"),
        7002 => Some("TERRAIN_BASE_COLUMNS"),
        7003 => Some("TERRAIN_LENGTH_UNITS"),
        7004 => Some("TERRAIN_WIDTH_UNITS"),
        7005 => Some("TERRAIN_SECTOR_ROWS"),
        7006 => Some("TERRAIN_SECTOR_COLUMNS"),
        7007 => Some("TERRAIN_TESSELLATION"),
        7008 => Some("TERRAIN_SECTOR_ID"),
        7009 => Some("TERRAIN_SECTOR_LIST"),
        7010 => Some("TERRAIN_MESHFACE_ID"),
        7011 => Some("TERRAIN_MESHFACE_LIST"),
        7012 => Some("TERRAIN_MESHEDGE_ID"),
        7013 => Some("TERRAIN_MESHEDGE_START_VERTEX"),
        7015 => Some("TERRAIN_MESHEDGE_SUBEDGE_LIST"),
        7016 => Some("TERRAIN_MESHEDGE_LIST"),
        7017 => Some("TERRAIN_SUBEDGE_ID"),
        7018 => Some("TERRAIN_MESHVERTEX_ID"),
        7019 => Some("TERRAIN_MESHVERTEX_POSITION"),
        7020 => Some("TERRAIN_MESHVERTEX_LEVEL"),
        7021 => Some("TERRAIN_MESHVERTEX_CONSTRAINT_LIST"),
        7022 => Some("TERRAIN_MESHVERTEX_CONSTRAINT_ID"),
        7023 => Some("TERRAIN_MESHVERTEX_LIST"),
        7024 => Some("TERRAIN_ELEMENT_ID_VALUE"),
        7025 => Some("TERRAIN_ELEMENT_ID_SECTOR"),
        7026 => Some("TERRAIN_MATERIAL_VALUE"),
        7027 => Some("TERRAIN_MATERIAL_LIST"),
        7028 => Some("TERRAIN_AREA_INFORMATION"),
        7029 => Some("TERRAIN_VERTEX_U"),
        7030 => Some("TERRAIN_VERTEX_V"),
        7037 => Some("TERRAIN_MAPVERTEX_ID"),
        7038 => Some("TERRAIN_MAPVERTEX_UVW"),
        7039 => Some("TERRAIN_MAPVERTEX_LIST"),
        7040 => Some("TERRAIN_MAPEDGE_ID"),
        7041 => Some("TERRAIN_MAPEDGE_START_VERTEX"),
        7042 => Some("TERRAIN_MAPEDGE_LIST"),
        7043 => Some("TERRAIN_MAPFACE_ID"),
        7044 => Some("TERRAIN_MAPFACE_LAYER"),
        7045 => Some("TERRAIN_MAPFACE_LIST"),
        7046 => Some("TERRAIN_MAPFACE_BLENDPAGE_ID"),
        7047 => Some("TERRAIN_BLENDWEIGHT_MATID"),
        7048 => Some("TERRAIN_BLENDWEIGHT_WEIGHT"),
        7049 => Some("TERRAIN_BLENDTEXEL_WEIGHTLIST"),
        7050 => Some("TERRAIN_BLENDPAGE_ID"),
        7051 => Some("TERRAIN_BLENDPAGE_WIDTH"),
        7052 => Some("TERRAIN_BLENDPAGE_HEIGHT"),
        7053 => Some("TERRAIN_BLENDPAGE_TEXEL_LIST"),
        7054 => Some("TERRAIN_BLENDPAGE_LIST"),
        7055 => Some("TERRAIN_MESH"),
        7056 => Some("TERRAIN_PALETTE"),
        7057 => Some("TERRAIN_BLENDTEXEL_BYTEWEIGHTLIST"),
        7058 => Some("TERRAIN_MESH_NAME"),
        7059 => Some("TERRAIN_PALETTE_NAME"),
        7060 => Some("TERRAIN_MATERIAL"),
        7061 => Some("TERRAIN_MATERIAL_ID"),
        7062 => Some("TERRAIN_MATERIAL_NAME"),
        7063 => Some("TERRAIN_MATERIAL_SCALE"),
        7064 => Some("TERRAIN_MATERIAL_DIFFUSE_NAME"),
        7065 => Some("TERRAIN_MATERIAL_NORMAL_NAME"),
        7066 => Some("TERRAIN_MATERIAL_SPECUALAR_NAME"),
        7067 => Some("TERRAIN_MATERIAL_HEIGHTMAP_NAME"),
        7068 => Some("TERRAIN_PALETTE_PARALLAX_GLOBAL"),
        7069 => Some("TERRAIN_MATERIAL_RELIEF_SCALE"),
        7070 => Some("TERRAIN_BLENDTEXEL_6BYTEWEIGHTLIST"),
        7071 => Some("TERRAIN_MESHVERTEX_CONSTRAINT_A"),
        7072 => Some("TERRAIN_MESHVERTEX_CONSTRAINT_B"),
        7073 => Some("TERRAIN_MESHEDGE_SUBEDGE_A"),
        7074 => Some("TERRAIN_MESHEDGE_SUBEDGE_B"),
        7075 => Some("TERRAIN_BLENDTEXEL_ID"),
        7076 => Some("TERRAIN_SOUND_DATA"),
        7077 => Some("TERRAIN_MATERIAL_SPECULAR_COLOR"),
        7900 => Some("WATER_INFORMATION"),
        7901 => Some("WATER_VERSION"),
        7902 => Some("WATER_ID"),
        7903 => Some("WATER_VERTEX_LIST"),
        7904 => Some("WATER_VERTEX_POSITION"),
        7905 => Some("WATER_VERTEX_NORMAL"),
        7906 => Some("WATER_VERTEX_UVW"),
        7907 => Some("WATER_VERTEX_COLOR"),
        7908 => Some("WATER_VERTEX_INDEX_LIST"),
        8000 => Some("MESH_CHUNK_VERTEXSIZE"),
        8001 => Some("MESH_CHUNK_VERTEXCOUNT"),
        8002 => Some("MESH_CHUNK_INDEXCOUNT"),
        8003 => Some("MESH_CHUNK_PRIMITIVETYPE"),
        8004 => Some("MESH_CHUNK_INDEXFORMAT"),
        8005 => Some("MESH_CHUNK_BASEVERTEXINDEX"),
        8006 => Some("MESH_CHUNK_VERTEXOFFSET"),
        8007 => Some("MESH_CHUNK_MININDEX"),
        8008 => Some("MESH_CHUNK_VERTICESREFERENCED"),
        8009 => Some("MESH_CHUNK_STARTINDEX"),
        8010 => Some("MESH_CHUNK_HASINSTGEOM"),
        8011 => Some("MESH_CHUNK_ADDITIONALSTREAMS"),
        8012 => Some("MESH_STREAM_VERTEXSIZE"),
        8013 => Some("MESH_STREAM_VERTEXCOUNT"),
        8014 => Some("MESH_STREAM_FREQUENCY"),
        8015 => Some("MESH_STREAM_LOOPING"),
        8016 => Some("MESH_STREAM_INSTANCED"),
        8017 => Some("MESH_BOUNDS_BOXMIN"),
        8018 => Some("MESH_BOUNDS_BOXMAX"),
        8019 => Some("MESH_BOUNDS_SPHERE"),
        8020 => Some("MESH_CHUNK_BOUNDS"),
        8021 => Some("MESH_CHUNKS"),
        8022 => Some("MESH_VERTEXDATA"),
        8023 => Some("MESH_INDEXDATA"),
        8024 => Some("MESH_STREAM_VERTEXDATA"),
        8025 => Some("MESH_CHUNK_VERTEXDECLARATOR"),
        8026 => Some("MESH_VERTEXDECLARATOR_STREAM"),
        8027 => Some("MESH_VERTEXDECLARATOR_OFFSET"),
        8028 => Some("MESH_VERTEXDECLARATOR_DATATYPE"),
        8029 => Some("MESH_VERTEXDECLARATOR_USAGE"),
        8030 => Some("MESH_VERTEXDECLARATOR_USAGEINDEX"),
        8031 => Some("MESH_VERTEXDECLARATOR_METHOD"),
        8032 => Some("MESH_INDEXFORMAT"),
        8033 => Some("MESH_INSTANCED_STREAM"),
        8034 => Some("MESH_CHUNK_INSTANCES_COUNT"),
        9000 => Some("AC_NODE_NAME"),
        9001 => Some("AC_EDGE_START_ID"),
        9002 => Some("AC_EDGE_END_ID"),
        9003 => Some("AC_CAPTION"),
        9004 => Some("AC_NODE_SOCKET_LIST"),
        9005 => Some("AC_SOCKET_IS_OUTPUT"),
        9006 => Some("AC_NODE_IMAGE"),
        9007 => Some("AC_EDGE_LIST"),
        9008 => Some("AC_NODE_LIST"),
        9009 => Some("AC_NODE_COLOUR"),
        9010 => Some("AC_NODE_ANIMATION"),
        9011 => Some("AC_CURVE_CONTROL_POINT_LIST"),
        9012 => Some("AC_CURVE_CONTROL_POINT_TIME"),
        9013 => Some("AC_CURVE_CONTROL_POINT_VALUE"),
        9014 => Some("AC_MODEL_NAME"),
        9015 => Some("AC_EVENT_LIST"),
        9016 => Some("AC_EVENT_TIME"),
        9017 => Some("AC_EVENT_ID"),
        9018 => Some("AC_NODE_LOOPING"),
        9019 => Some("AC_FLAGS"),
        9020 => Some("AC_TRANS_ANIM_NAME"),
        9021 => Some("AC_TRANS_ANIM_START"),
        9022 => Some("AC_TRANS_ANIM_LENGTH"),
        9023 => Some("AC_TRANS_TRACK_LIST"),
        9024 => Some("AC_TRANSITION_LIST"),
        9025 => Some("AC_TRANS_LENGTH"),
        9100 => Some("AC_BLENDGROUP_ANIM_LIST"),
        9101 => Some("AC_BLEND_GROUP_LIST"),
        9102 => Some("AC_BLENDGROUP_NAME"),
        10000 => Some("G2DA_COLUMN_NAME"),
        10001 => Some("G2DA_COLUMN_HASH"),
        10002 => Some("G2DA_COLUMN_LIST"),
        10003 => Some("G2DA_ROW_LIST"),
        10004 => Some("G2DA_ROW_DATA"),
        10005 => Some("G2DA_COLUMN_1"),
        10006 => Some("G2DA_COLUMN_2"),
        10007 => Some("G2DA_COLUMN_3"),
        10008 => Some("G2DA_COLUMN_4"),
        10009 => Some("G2DA_COLUMN_5"),
        10010 => Some("G2DA_COLUMN_6"),
        10011 => Some("G2DA_COLUMN_7"),
        10012 => Some("G2DA_COLUMN_8"),
        10013 => Some("G2DA_COLUMN_9"),
        10014 => Some("G2DA_COLUMN_10"),
        10015 => Some("G2DA_COLUMN_11"),
        10016 => Some("G2DA_COLUMN_12"),
        10017 => Some("G2DA_COLUMN_13"),
        10018 => Some("G2DA_COLUMN_14"),
        10019 => Some("G2DA_COLUMN_15"),
        10020 => Some("G2DA_COLUMN_16"),
        10021 => Some("G2DA_COLUMN_17"),
        10022 => Some("G2DA_COLUMN_18"),
        10023 => Some("G2DA_COLUMN_19"),
        10024 => Some("G2DA_COLUMN_20"),
        10025 => Some("G2DA_COLUMN_21"),
        10026 => Some("G2DA_COLUMN_22"),
        10027 => Some("G2DA_COLUMN_23"),
        10028 => Some("G2DA_COLUMN_24"),
        10029 => Some("G2DA_COLUMN_25"),
        10030 => Some("G2DA_COLUMN_26"),
        10031 => Some("G2DA_COLUMN_27"),
        10032 => Some("G2DA_COLUMN_28"),
        10033 => Some("G2DA_COLUMN_29"),
        10034 => Some("G2DA_COLUMN_30"),
        10035 => Some("G2DA_COLUMN_31"),
        10036 => Some("G2DA_COLUMN_32"),
        10037 => Some("G2DA_COLUMN_33"),
        10038 => Some("G2DA_COLUMN_34"),
        10039 => Some("G2DA_COLUMN_35"),
        10040 => Some("G2DA_COLUMN_36"),
        10041 => Some("G2DA_COLUMN_37"),
        10042 => Some("G2DA_COLUMN_38"),
        10043 => Some("G2DA_COLUMN_39"),
        10044 => Some("G2DA_COLUMN_40"),
        10045 => Some("G2DA_COLUMN_41"),
        10046 => Some("G2DA_COLUMN_42"),
        10047 => Some("G2DA_COLUMN_43"),
        10048 => Some("G2DA_COLUMN_44"),
        10049 => Some("G2DA_COLUMN_45"),
        10050 => Some("G2DA_COLUMN_46"),
        10051 => Some("G2DA_COLUMN_47"),
        10052 => Some("G2DA_COLUMN_48"),
        10053 => Some("G2DA_COLUMN_49"),
        10054 => Some("G2DA_COLUMN_50"),
        10055 => Some("G2DA_COLUMN_51"),
        10056 => Some("G2DA_COLUMN_52"),
        10057 => Some("G2DA_COLUMN_53"),
        10058 => Some("G2DA_COLUMN_54"),
        10059 => Some("G2DA_COLUMN_55"),
        10060 => Some("G2DA_COLUMN_56"),
        10061 => Some("G2DA_COLUMN_57"),
        10062 => Some("G2DA_COLUMN_58"),
        10063 => Some("G2DA_COLUMN_59"),
        10064 => Some("G2DA_COLUMN_60"),
        10065 => Some("G2DA_COLUMN_61"),
        10066 => Some("G2DA_COLUMN_62"),
        10067 => Some("G2DA_COLUMN_63"),
        10068 => Some("G2DA_COLUMN_64"),
        10069 => Some("G2DA_COLUMN_65"),
        10070 => Some("G2DA_COLUMN_66"),
        10071 => Some("G2DA_COLUMN_67"),
        10072 => Some("G2DA_COLUMN_68"),
        10073 => Some("G2DA_COLUMN_69"),
        10074 => Some("G2DA_COLUMN_70"),
        10075 => Some("G2DA_COLUMN_71"),
        10076 => Some("G2DA_COLUMN_72"),
        10077 => Some("G2DA_COLUMN_73"),
        10078 => Some("G2DA_COLUMN_74"),
        10079 => Some("G2DA_COLUMN_75"),
        10080 => Some("G2DA_COLUMN_76"),
        10081 => Some("G2DA_COLUMN_77"),
        10082 => Some("G2DA_COLUMN_78"),
        10083 => Some("G2DA_COLUMN_79"),
        10084 => Some("G2DA_COLUMN_80"),
        10085 => Some("G2DA_COLUMN_81"),
        10086 => Some("G2DA_COLUMN_82"),
        10087 => Some("G2DA_COLUMN_83"),
        10088 => Some("G2DA_COLUMN_84"),
        10089 => Some("G2DA_COLUMN_85"),
        10090 => Some("G2DA_COLUMN_86"),
        10091 => Some("G2DA_COLUMN_87"),
        10092 => Some("G2DA_COLUMN_88"),
        10093 => Some("G2DA_COLUMN_89"),
        10094 => Some("G2DA_COLUMN_90"),
        10095 => Some("G2DA_COLUMN_91"),
        10096 => Some("G2DA_COLUMN_92"),
        10097 => Some("G2DA_COLUMN_93"),
        10098 => Some("G2DA_COLUMN_94"),
        10099 => Some("G2DA_COLUMN_95"),
        10100 => Some("G2DA_COLUMN_96"),
        10101 => Some("G2DA_COLUMN_97"),
        10102 => Some("G2DA_COLUMN_98"),
        10103 => Some("G2DA_COLUMN_99"),
        10104 => Some("G2DA_COLUMN_100"),
        10105 => Some("G2DA_COLUMN_101"),
        10106 => Some("G2DA_COLUMN_102"),
        10107 => Some("G2DA_COLUMN_103"),
        10108 => Some("G2DA_COLUMN_104"),
        10109 => Some("G2DA_COLUMN_105"),
        10110 => Some("G2DA_COLUMN_106"),
        10111 => Some("G2DA_COLUMN_107"),
        10112 => Some("G2DA_COLUMN_108"),
        10113 => Some("G2DA_COLUMN_109"),
        10114 => Some("G2DA_COLUMN_110"),
        10115 => Some("G2DA_COLUMN_111"),
        10116 => Some("G2DA_COLUMN_112"),
        10117 => Some("G2DA_COLUMN_113"),
        10118 => Some("G2DA_COLUMN_114"),
        10119 => Some("G2DA_COLUMN_115"),
        10120 => Some("G2DA_COLUMN_116"),
        10121 => Some("G2DA_COLUMN_117"),
        10122 => Some("G2DA_COLUMN_118"),
        10123 => Some("G2DA_COLUMN_119"),
        10124 => Some("G2DA_COLUMN_120"),
        10125 => Some("G2DA_COLUMN_121"),
        10126 => Some("G2DA_COLUMN_122"),
        10127 => Some("G2DA_COLUMN_123"),
        10128 => Some("G2DA_COLUMN_124"),
        10129 => Some("G2DA_COLUMN_125"),
        10130 => Some("G2DA_COLUMN_126"),
        10131 => Some("G2DA_COLUMN_127"),
        10132 => Some("G2DA_COLUMN_128"),
        10133 => Some("G2DA_COLUMN_129"),
        10134 => Some("G2DA_COLUMN_130"),
        10135 => Some("G2DA_COLUMN_131"),
        10136 => Some("G2DA_COLUMN_132"),
        10137 => Some("G2DA_COLUMN_133"),
        10138 => Some("G2DA_COLUMN_134"),
        10139 => Some("G2DA_COLUMN_135"),
        10140 => Some("G2DA_COLUMN_136"),
        10141 => Some("G2DA_COLUMN_137"),
        10142 => Some("G2DA_COLUMN_138"),
        10143 => Some("G2DA_COLUMN_139"),
        10144 => Some("G2DA_COLUMN_140"),
        10145 => Some("G2DA_COLUMN_141"),
        10146 => Some("G2DA_COLUMN_142"),
        10147 => Some("G2DA_COLUMN_143"),
        10148 => Some("G2DA_COLUMN_144"),
        10149 => Some("G2DA_COLUMN_145"),
        10150 => Some("G2DA_COLUMN_146"),
        10151 => Some("G2DA_COLUMN_147"),
        10152 => Some("G2DA_COLUMN_148"),
        10153 => Some("G2DA_COLUMN_149"),
        10154 => Some("G2DA_COLUMN_150"),
        10155 => Some("G2DA_COLUMN_151"),
        10156 => Some("G2DA_COLUMN_152"),
        10157 => Some("G2DA_COLUMN_153"),
        10158 => Some("G2DA_COLUMN_154"),
        10159 => Some("G2DA_COLUMN_155"),
        10160 => Some("G2DA_COLUMN_156"),
        10161 => Some("G2DA_COLUMN_157"),
        10162 => Some("G2DA_COLUMN_158"),
        10163 => Some("G2DA_COLUMN_159"),
        10164 => Some("G2DA_COLUMN_160"),
        10165 => Some("G2DA_COLUMN_161"),
        10166 => Some("G2DA_COLUMN_162"),
        10167 => Some("G2DA_COLUMN_163"),
        10168 => Some("G2DA_COLUMN_164"),
        10169 => Some("G2DA_COLUMN_165"),
        10170 => Some("G2DA_COLUMN_166"),
        10171 => Some("G2DA_COLUMN_167"),
        10172 => Some("G2DA_COLUMN_168"),
        10173 => Some("G2DA_COLUMN_169"),
        10174 => Some("G2DA_COLUMN_170"),
        10175 => Some("G2DA_COLUMN_171"),
        10176 => Some("G2DA_COLUMN_172"),
        10177 => Some("G2DA_COLUMN_173"),
        10178 => Some("G2DA_COLUMN_174"),
        10179 => Some("G2DA_COLUMN_175"),
        10180 => Some("G2DA_COLUMN_176"),
        10181 => Some("G2DA_COLUMN_177"),
        10182 => Some("G2DA_COLUMN_178"),
        10183 => Some("G2DA_COLUMN_179"),
        10184 => Some("G2DA_COLUMN_180"),
        10185 => Some("G2DA_COLUMN_181"),
        10186 => Some("G2DA_COLUMN_182"),
        10187 => Some("G2DA_COLUMN_183"),
        10188 => Some("G2DA_COLUMN_184"),
        10189 => Some("G2DA_COLUMN_185"),
        10190 => Some("G2DA_COLUMN_186"),
        10191 => Some("G2DA_COLUMN_187"),
        10192 => Some("G2DA_COLUMN_188"),
        10193 => Some("G2DA_COLUMN_189"),
        10194 => Some("G2DA_COLUMN_190"),
        10195 => Some("G2DA_COLUMN_191"),
        10196 => Some("G2DA_COLUMN_192"),
        10197 => Some("G2DA_COLUMN_193"),
        10198 => Some("G2DA_COLUMN_194"),
        10199 => Some("G2DA_COLUMN_195"),
        10200 => Some("G2DA_COLUMN_196"),
        10201 => Some("G2DA_COLUMN_197"),
        10202 => Some("G2DA_COLUMN_198"),
        10203 => Some("G2DA_COLUMN_199"),
        10204 => Some("G2DA_COLUMN_200"),
        10205 => Some("G2DA_COLUMN_201"),
        10206 => Some("G2DA_COLUMN_202"),
        10207 => Some("G2DA_COLUMN_203"),
        10208 => Some("G2DA_COLUMN_204"),
        10209 => Some("G2DA_COLUMN_205"),
        10210 => Some("G2DA_COLUMN_206"),
        10211 => Some("G2DA_COLUMN_207"),
        10212 => Some("G2DA_COLUMN_208"),
        10213 => Some("G2DA_COLUMN_209"),
        10214 => Some("G2DA_COLUMN_210"),
        10215 => Some("G2DA_COLUMN_211"),
        10216 => Some("G2DA_COLUMN_212"),
        10217 => Some("G2DA_COLUMN_213"),
        10218 => Some("G2DA_COLUMN_214"),
        10219 => Some("G2DA_COLUMN_215"),
        10220 => Some("G2DA_COLUMN_216"),
        10221 => Some("G2DA_COLUMN_217"),
        10222 => Some("G2DA_COLUMN_218"),
        10223 => Some("G2DA_COLUMN_219"),
        10224 => Some("G2DA_COLUMN_220"),
        10225 => Some("G2DA_COLUMN_221"),
        10226 => Some("G2DA_COLUMN_222"),
        10227 => Some("G2DA_COLUMN_223"),
        10228 => Some("G2DA_COLUMN_224"),
        10229 => Some("G2DA_COLUMN_225"),
        10230 => Some("G2DA_COLUMN_226"),
        10231 => Some("G2DA_COLUMN_227"),
        10232 => Some("G2DA_COLUMN_228"),
        10233 => Some("G2DA_COLUMN_229"),
        10234 => Some("G2DA_COLUMN_230"),
        10235 => Some("G2DA_COLUMN_231"),
        10236 => Some("G2DA_COLUMN_232"),
        10237 => Some("G2DA_COLUMN_233"),
        10238 => Some("G2DA_COLUMN_234"),
        10239 => Some("G2DA_COLUMN_235"),
        10240 => Some("G2DA_COLUMN_236"),
        10241 => Some("G2DA_COLUMN_237"),
        10242 => Some("G2DA_COLUMN_238"),
        10243 => Some("G2DA_COLUMN_239"),
        10244 => Some("G2DA_COLUMN_240"),
        10245 => Some("G2DA_COLUMN_241"),
        10246 => Some("G2DA_COLUMN_242"),
        10247 => Some("G2DA_COLUMN_243"),
        10248 => Some("G2DA_COLUMN_244"),
        10249 => Some("G2DA_COLUMN_245"),
        10250 => Some("G2DA_COLUMN_246"),
        10251 => Some("G2DA_COLUMN_247"),
        10252 => Some("G2DA_COLUMN_248"),
        10253 => Some("G2DA_COLUMN_249"),
        10254 => Some("G2DA_COLUMN_250"),
        10255 => Some("G2DA_COLUMN_251"),
        10256 => Some("G2DA_COLUMN_252"),
        10257 => Some("G2DA_COLUMN_253"),
        10258 => Some("G2DA_COLUMN_254"),
        10259 => Some("G2DA_COLUMN_255"),
        10260 => Some("G2DA_COLUMN_256"),
        10261 => Some("G2DA_COLUMN_257"),
        10262 => Some("G2DA_COLUMN_258"),
        10263 => Some("G2DA_COLUMN_259"),
        10264 => Some("G2DA_COLUMN_260"),
        10265 => Some("G2DA_COLUMN_261"),
        10266 => Some("G2DA_COLUMN_262"),
        10267 => Some("G2DA_COLUMN_263"),
        10268 => Some("G2DA_COLUMN_264"),
        10269 => Some("G2DA_COLUMN_265"),
        10270 => Some("G2DA_COLUMN_266"),
        10271 => Some("G2DA_COLUMN_267"),
        10272 => Some("G2DA_COLUMN_268"),
        10273 => Some("G2DA_COLUMN_269"),
        10274 => Some("G2DA_COLUMN_270"),
        10275 => Some("G2DA_COLUMN_271"),
        10276 => Some("G2DA_COLUMN_272"),
        10277 => Some("G2DA_COLUMN_273"),
        10278 => Some("G2DA_COLUMN_274"),
        10279 => Some("G2DA_COLUMN_275"),
        10280 => Some("G2DA_COLUMN_276"),
        10281 => Some("G2DA_COLUMN_277"),
        10282 => Some("G2DA_COLUMN_278"),
        10283 => Some("G2DA_COLUMN_279"),
        10284 => Some("G2DA_COLUMN_280"),
        10285 => Some("G2DA_COLUMN_281"),
        10286 => Some("G2DA_COLUMN_282"),
        10287 => Some("G2DA_COLUMN_283"),
        10288 => Some("G2DA_COLUMN_284"),
        10289 => Some("G2DA_COLUMN_285"),
        10290 => Some("G2DA_COLUMN_286"),
        10291 => Some("G2DA_COLUMN_287"),
        10292 => Some("G2DA_COLUMN_288"),
        10293 => Some("G2DA_COLUMN_289"),
        10294 => Some("G2DA_COLUMN_290"),
        10295 => Some("G2DA_COLUMN_291"),
        10296 => Some("G2DA_COLUMN_292"),
        10297 => Some("G2DA_COLUMN_293"),
        10298 => Some("G2DA_COLUMN_294"),
        10299 => Some("G2DA_COLUMN_295"),
        10300 => Some("G2DA_COLUMN_296"),
        10301 => Some("G2DA_COLUMN_297"),
        10302 => Some("G2DA_COLUMN_298"),
        10303 => Some("G2DA_COLUMN_299"),
        10999 => Some("G2DA_COLUMN_TYPE"),
        11000 => Some("STAGE_PLACE_LIST"),
        11001 => Some("STAGE_CAMERA_LIST"),
        11002 => Some("STAGE_PLACES_IN_SHOT"),
        11003 => Some("STAGE_CAMERA_FOV"),
        11004 => Some("STAGE_PLACE_DEFAULT_CAMERA"),
        11005 => Some("STAGE_CAMERA_DEPRECATED"),
        11006 => Some("STAGE_CAMERA_LOOKING_FROM"),
        11007 => Some("STAGE_CAMERA_LOOKING_AT_PRIMARY"),
        11008 => Some("STAGE_CAMERA_LOOKING_AT_SECONDARY"),
        11009 => Some("STAGE_CAMERA_LOOKING_AT_TYPE"),
        12000 => Some("CONVERSATION_STARTING_LIST"),
        12001 => Some("CONVERSATION_STARTING_INDEX"),
        12002 => Some("CONVERSATION_LINE_LIST"),
        12003 => Some("CONVERSATION_END"),
        12004 => Some("CONVERSATION_VOBANK"),
        12100 => Some("CONVERSATION_STAGE_NAME"),
        12101 => Some("CONVERSATION_STAGE_MAP"),
        12102 => Some("CONVERSATION_KEY_TAG"),
        12103 => Some("CONVERSATION_VALUE_TAG"),
        12104 => Some("CONVERSATION_STAGE_AT_CURRENT_LOCATION"),
        12201 => Some("CONVERSATION_LINE_TEXT"),
        12202 => Some("CONVERSATION_LINE_SPEAKER"),
        12203 => Some("CONVERSATION_LINE_LISTENER"),
        12204 => Some("CONVERSATION_LINE_GAME_LANGUAGE"),
        12205 => Some("CONVERSATION_LINE_ICON"),
        12206 => Some("CONVERSATION_LINE_VISIBILITY"),
        12207 => Some("CONVERSATION_LINE_AMBIENT"),
        12208 => Some("CONVERSATION_LINE_COND"),
        12209 => Some("CONVERSATION_LINE_ACTION"),
        12210 => Some("CONVERSATION_LINE_CUTSCENE_RESREF"),
        12211 => Some("CONVERSATION_LINE_CUTSCENE"),
        12212 => Some("CONVERSATION_LINE_CUTSCENE_MAP"),
        12213 => Some("CONVERSATION_LINE_ANIMATION"),
        12214 => Some("CONVERSATION_LINE_SKIP"),
        12215 => Some("CONVERSATION_LINE_FASTPATH"),
        12216 => Some("CONVERSATION_LINE_NOVOINGAME"),
        12217 => Some("CONVERSATION_LINE_REVERT_ANIM"),
        12218 => Some("CONVERSATION_LINE_SLIDE_SHOW_TEXTURE"),
        12300 => Some("CONVERSATION_PLOT_GUID"),
        12301 => Some("CONVERSATION_PLOT_FLAG"),
        12302 => Some("CONVERSATION_PLOT_TEST"),
        12303 => Some("CONVERSATION_SCRIPT"),
        12304 => Some("CONVERSATION_SCRIPT_PARAMETER"),
        12400 => Some("CONVERSATION_LINE_CHILDREN_LIST"),
        12500 => Some("CONVERSATION_LINE_ACTIVE"),
        13000 => Some("PLOT_FLAGS"),
        13001 => Some("PLOT_FLAG_ID"),
        13002 => Some("PLOT_FLAG_NAME"),
        13003 => Some("PLOT_FLAG_REWARD"),
        13004 => Some("PLOT_FLAG_JOURNAL"),
        13005 => Some("PLOT_FLAG_ENDS_PLOT"),
        13006 => Some("PLOT_FLAG_MULTIREWARD"),
        13007 => Some("PLOT_GUID"),
        13008 => Some("PLOT_NAME"),
        13009 => Some("PLOT_SCRIPT"),
        13010 => Some("PLOT_PRIORITY"),
        13011 => Some("PLOT_FLAGS1"),
        13012 => Some("PLOT_FLAGS2"),
        13013 => Some("PLOT_FLAGS3"),
        13014 => Some("PLOT_FLAGS4"),
        13015 => Some("PLOT_JOURNAL_IMAGE"),
        13016 => Some("PLOT_PLOTS"),
        13017 => Some("PLOT_PARENT_PLOT"),
        13018 => Some("PLOT_FLAG_AREA_LOCATION_TAG"),
        13019 => Some("PLOTASSIST_LIST"),
        13020 => Some("PLOTASSIST_TAG"),
        13021 => Some("PLOTASSIST_ADVANCES_PLOT"),
        13022 => Some("PLOT_ENTRYTYPE"),
        13023 => Some("PLOT_ALLOW_PAUSING"),
        13024 => Some("PLOT_FLAG_OFFERID"),
        13025 => Some("PLOT_PARENT_PLOT_GUID"),
        14000 => Some("TINT_MASK_DIFFUSE_R"),
        14001 => Some("TINT_MASK_DIFFUSE_G"),
        14002 => Some("TINT_MASK_DIFFUSE_B"),
        14003 => Some("TINT_MASK_SPECULAR_R"),
        14004 => Some("TINT_MASK_SPECULAR_G"),
        14005 => Some("TINT_MASK_SPECULAR_B"),
        14006 => Some("TINT_MASK_DIFFUSE_A"),
        14007 => Some("TINT_MASK_SPECULAR_A"),
        14008 => Some("TINT_MASK_DIFFUSE_OPACITY"),
        14009 => Some("TINT_MASK_SPECULAR_OPACITY"),
        15000 => Some("MAT_FILE_OBJECT_VERSION"),
        15001 => Some("MAT_CHILD_LIST"),
        15010 => Some("MAT_ROOT"),
        15011 => Some("MAT_ROOT_NAME"),
        15012 => Some("MAT_MODEL"),
        15013 => Some("MAT_MODEL_NAME"),
        15014 => Some("MAT_PART"),
        15015 => Some("MAT_PART_NAME"),
        15016 => Some("MAT_PART_MMH_PARENT"),
        15017 => Some("MAT_MATLIB"),
        15018 => Some("MAT_MATLIB_NAME"),
        15019 => Some("MAT_MATOBJ"),
        15020 => Some("MAT_MATOBJ_NAME"),
        15021 => Some("MAT_LIGHT"),
        15022 => Some("MAT_LIGHT_NAME"),
        15023 => Some("MAT_LIGHT_RIG"),
        15024 => Some("MAT_LIGHT_RIG_NAME"),
        15025 => Some("MAT_LIGHT_PROBE"),
        15026 => Some("MAT_LIGHT_PROBE_NAME"),
        15027 => Some("MAT_GROUP"),
        15028 => Some("MAT_GROUP_NAME"),
        15029 => Some("MAT_PALETTELIB"),
        15030 => Some("MAT_PALETTELIB_NAME"),
        15031 => Some("MAT_PALETTEOBJ"),
        15032 => Some("MAT_PALETTEOBJ_NAME"),
        15033 => Some("MAT_HERALDRYLIB"),
        15034 => Some("MAT_HERALDRYLIB_NAME"),
        15035 => Some("MAT_HERALDRYOBJ"),
        15036 => Some("MAT_HERALDRYOBJ_NAME"),
        15037 => Some("MAT_DUPLICATE"),
        15038 => Some("MAT_DUPLICATE_NAME"),
        15039 => Some("MAT_LAYOUT_NAME"),
        15040 => Some("MAT_TINTLIB"),
        15041 => Some("MAT_TINTLIB_NAME"),
        15042 => Some("MAT_TINTOBJ"),
        15043 => Some("MAT_TINTOBJ_NAME"),
        15050 => Some("MAT_MATERIAL_TYPE"),
        15051 => Some("MAT_BASIC_PARAMS"),
        15052 => Some("MAT_SHINY_TRANS"),
        15053 => Some("MAT_TWO_SIDE"),
        15054 => Some("MAT_HAIR"),
        15055 => Some("MAT_DYNC_LIGHT"),
        15056 => Some("MAT_BLEND_MODE"),
        15057 => Some("MAT_NAME"),
        15058 => Some("MAT_MATERIAL_TYPE_STRING"),
        15059 => Some("MAT_MATERIAL_SEMANTIC"),
        15060 => Some("MAT_MATERIAL_SOUND_TYPE"),
        15070 => Some("MAT_DIFFUSE_MAP_TYPE"),
        15071 => Some("MAT_DIFFUSE_MAP_COLOR"),
        15072 => Some("MAT_DIFFUSE_MAP_SCALE"),
        15073 => Some("MAT_DIFFUSE_MAP"),
        15074 => Some("MAT_DIFFUSE_FILENAME"),
        15075 => Some("MAT_DIFFOPAC_DIMENSIONX"),
        15076 => Some("MAT_DIFFOPAC_DIMENSIONY"),
        15077 => Some("MAT_DIFFOPAC_COMPRESSION"),
        15078 => Some("MAT_DIFFOPAC_COMPRESSION_XBOX360"),
        15080 => Some("MAT_SECONDARY_DIFFUSE_MAP_ENABLE"),
        15081 => Some("MAT_SECONDARY_DIFFUSE_MAP"),
        15082 => Some("MAT_SECONDARY_DIFFUSE_FILENAME"),
        15085 => Some("MAT_SECONDARY_DIFFUSE_COMPRESSION"),
        15086 => Some("MAT_SECONDARY_DIFFUSE_COMPRESSION_XBOX360"),
        15100 => Some("MAT_OPACITYMAPENABLE"),
        15101 => Some("MAT_OPACITYMAPTYPE"),
        15102 => Some("MAT_OPACITYMAPCOLOR"),
        15103 => Some("MAT_OPACITYMAPSCALE"),
        15104 => Some("MAT_OPACITYMAP"),
        15130 => Some("MAT_SPECULAR_MAP_ENABLE"),
        15131 => Some("MAT_SPECULAR_MAP_TYPE"),
        15132 => Some("MAT_SPECULAR_MAP_COLOR"),
        15133 => Some("MAT_SPECULAR_MAP_SCALE"),
        15134 => Some("MAT_SPECULAR_MAP"),
        15135 => Some("MAT_SPECULAR_GLOSS_TYPE"),
        15136 => Some("MAT_SPECULAR_GLOSS_COLOR"),
        15137 => Some("MAT_SPECULAR_GLOSS_SCALE"),
        15138 => Some("MAT_SPECULAR_GLOSS"),
        15139 => Some("MAT_SPECULAR_FILENAME"),
        15140 => Some("MAT_SPECULAR_DIMENSIONX"),
        15141 => Some("MAT_SPECULAR_DIMENSIONY"),
        15142 => Some("MAT_SPECULAR_COMPRESSION"),
        15143 => Some("MAT_SPECULAR_COMPRESSION_XBOX360"),
        15144 => Some("MAT_SPECULAR_REFLECTION_MULTIPLIER"),
        15160 => Some("MAT_NORMAL_MAP_ENABLE"),
        15161 => Some("MAT_NORMAL_MAP"),
        15162 => Some("MAT_NORMAL_FILENAME"),
        15163 => Some("MAT_NORMAL_COMPRESSION"),
        15164 => Some("MAT_NORMAL_COMPRESSION_XBOX360"),
        15190 => Some("MAT_TINT_MAP_ENABLE"),
        15191 => Some("MAT_TINT_MAP"),
        15192 => Some("MAT_TINT_R_ENABLE"),
        15193 => Some("MAT_TINT_G_ENABLE"),
        15194 => Some("MAT_TINT_B_ENABLE"),
        15195 => Some("MAT_TINT_FILENAME_POSTFIX"),
        15196 => Some("MAT_TINT_COMPRESSION"),
        15197 => Some("MAT_TINT_COMPRESSION_XBOX360"),
        15198 => Some("MAT_TINT_A_ENABLE"),
        15199 => Some("MAT_TINT_R_SPECULAR_INTENSITY"),
        15200 => Some("MAT_TINT_G_SPECULAR_INTENSITY"),
        15201 => Some("MAT_TINT_B_SPECULAR_INTENSITY"),
        15202 => Some("MAT_TINT_A_SPECULAR_INTENSITY"),
        15203 => Some("MAT_TINT_R_DIFFUSE_INTENSITY"),
        15204 => Some("MAT_TINT_G_DIFFUSE_INTENSITY"),
        15205 => Some("MAT_TINT_B_DIFFUSE_INTENSITY"),
        15206 => Some("MAT_TINT_A_DIFFUSE_INTENSITY"),
        15207 => Some("MAT_TINT_R_SPECULAR_OPACITY"),
        15208 => Some("MAT_TINT_G_SPECULAR_OPACITY"),
        15209 => Some("MAT_TINT_B_SPECULAR_OPACITY"),
        15210 => Some("MAT_TINT_A_SPECULAR_OPACITY"),
        15211 => Some("MAT_TINT_R_DIFFUSE_OPACITY"),
        15212 => Some("MAT_TINT_G_DIFFUSE_OPACITY"),
        15213 => Some("MAT_TINT_B_DIFFUSE_OPACITY"),
        15214 => Some("MAT_TINT_A_DIFFUSE_OPACITY"),
        15215 => Some("MAT_TINT_TYPE"),
        15216 => Some("MAT_TINT_MASK_TINT_CHANNEL1"),
        15217 => Some("MAT_TINT_MASK_TINT_CHANNEL2"),
        15218 => Some("MAT_TINT_MASK_TINT_CHANNEL3"),
        15219 => Some("MAT_TINT_MASK_TINT_CHANNEL4"),
        15220 => Some("MAT_RELIEF_MAP_ENABLE"),
        15221 => Some("MAT_RELIEF_MAP"),
        15222 => Some("MAT_RELIEF_MAP_SCALE"),
        15223 => Some("MAT_RELIEF_MAP_SAMPLES"),
        15224 => Some("MAT_RELIEF_MAP_SHADOW_OFFSET"),
        15225 => Some("MAT_RELIEF_MAP_IN_OUT"),
        15226 => Some("MAT_RELIEF_COMPRESSION"),
        15227 => Some("MAT_RELIEF_COMPRESSION_XBOX360"),
        15228 => Some("MAT_TINT_EXPORTABLE"),
        15250 => Some("MAT_VFX_CONTACT_SHEET_WIDTH"),
        15251 => Some("MAT_VFX_CONTACT_SHEET_HEIGHT"),
        15252 => Some("MAT_VFX_CONTACT_SHEET_FRAMES"),
        15253 => Some("MAT_VFX_SCROLL_SPEED_U"),
        15254 => Some("MAT_VFX_SCROLL_SPEED_V"),
        15255 => Some("MAT_VFX_DEPTH_BIAS_ALPHA"),
        15256 => Some("MAT_VFX_START_ALPHA_FRESNEL"),
        15257 => Some("MAT_VFX_END_ALPHA_FRESNEL"),
        15258 => Some("MAT_VFX_INVERT_ALPHA_FRESNEL"),
        15280 => Some("MAT_FRESNEL_MAP_ENABLE"),
        15281 => Some("MAT_FRESNEL_MAP"),
        15282 => Some("MAT_FRESNEL_FILENAME"),
        15283 => Some("MAT_FRESNEL_COMPRESSION"),
        15284 => Some("MAT_FRESNEL_COMPRESSION_XBOX360"),
        15310 => Some("MAT_EMISSIVE_MAP_ENABLE"),
        15311 => Some("MAT_EMISSIVE_MAP"),
        15312 => Some("MAT_EMISSIVE_FILENAME"),
        15313 => Some("MAT_EMISSIVE_COMPRESSION"),
        15314 => Some("MAT_EMISSIVE_COMPRESSION_XBOX360"),
        15340 => Some("MAT_SECTION_MASK_MAP_ENABLE"),
        15341 => Some("MAT_SECTION_MASK_MAP"),
        15342 => Some("MAT_SECTION_MASK_FILENAME"),
        15343 => Some("MAT_SECTION_MASK_COMPRESSION"),
        15344 => Some("MAT_SECTION_MASK_COMPRESSION_XBOX360"),
        15360 => Some("MAT_SECONDARY_NORMAL_MAP_ENABLE"),
        15361 => Some("MAT_SECONDARY_NORMAL_MAP"),
        15362 => Some("MAT_SECONDARY_NORMAL_FILENAME"),
        15363 => Some("MAT_SECONDARY_NORMAL_COMPRESSION"),
        15364 => Some("MAT_SECONDARY_NORMAL_COMPRESSION_XBOX360"),
        15380 => Some("MAT_EYE_CORNEA_SPECULAR_MASK"),
        15381 => Some("MAT_EYE_CORNEA_SPECULAR_POWER"),
        15382 => Some("MAT_EYE_SCLERA_SPECULAR_MASK"),
        15383 => Some("MAT_EYE_SCLERA_SPECULAR_POWER"),
        15384 => Some("MAT_EYE_CORNEA_REFLECTION_MULTIPLIER"),
        15400 => Some("MAT_SPECULAR_MASK_MAP_ENABLE"),
        15401 => Some("MAT_PACKED_TEXTURE_MAP"),
        15402 => Some("MAT_PACKED_TEXTURE_FILENAME"),
        15403 => Some("MAT_PACKED_TEXTURE_COMPRESSION"),
        15404 => Some("MAT_PACKED_TEXTURE_COMPRESSION_XBOX360"),
        15420 => Some("MAT_SPECULAR_SHIFT_MAP_ENABLE"),
        15421 => Some("MAT_TINT_NOISE_MAP"),
        15422 => Some("MAT_TINT_NOISE_FILENAME"),
        15423 => Some("MAT_TINT_NOISE_COMPRESSION"),
        15424 => Some("MAT_TINT_NOISE_COMPRESSION_XBOX360"),
        15440 => Some("MAT_HAIR_DIFFUSE_TINT"),
        15441 => Some("MAT_HAIR_PRIMARY_SPECULAR_POWER"),
        15442 => Some("MAT_HAIR_PRIMARY_SPECULAR_MASK"),
        15443 => Some("MAT_HAIR_SECONDARY_SPECULAR_POWER"),
        15444 => Some("MAT_HAIR_SECONDARY_SPECULAR_TINT"),
        15445 => Some("MAT_HAIR_TINT_NOISE_TILING"),
        15460 => Some("MAT_SUN"),
        15461 => Some("MAT_SUN_NAME"),
        15462 => Some("MAT_SUN_DIRECTION"),
        15463 => Some("MAT_SUN_COLOR"),
        15464 => Some("MAT_SUN_COLORMULT"),
        15480 => Some("MAT_HERALDRY_MAP_ENABLE"),
        15481 => Some("MAT_HERALDRY_MAP"),
        15482 => Some("MAT_HERALDRY_FILENAME"),
        15483 => Some("MAT_HERALDRY_COMPRESSION"),
        15484 => Some("MAT_HERALDRY_COMPRESSION_XBOX360"),
        15500 => Some("MAT_RIM_LIGHT_WIDTH"),
        15501 => Some("MAT_RIM_LIGHT_MULTIPLIER"),
        15502 => Some("MAT_FALLOFF_WIDTH"),
        15503 => Some("MAT_FALLOFF_MULTIPLIER"),
        15510 => Some("MAT_AMBIENT_MULTIPLIER"),
        15511 => Some("MAT_SPECULAR_MULTIPLIER"),
        15512 => Some("MAT_LIP_SPECULAR_BOOST"),
        15513 => Some("MAT_RIM_POWER"),
        15520 => Some("MAT_DISTORTION_MAP_ENABLE"),
        15521 => Some("MAT_DISTORTION_MAP"),
        15522 => Some("MAT_DISTORTION_FILENAME"),
        15523 => Some("MAT_DISTORTION_COMPRESSION"),
        15524 => Some("MAT_DISTORTION_COMPRESSION_XBOX360"),
        15540 => Some("MAT_DISTORTIONMODIFIERS_MAP_ENABLE"),
        15541 => Some("MAT_DISTORTIONMODIFIERS_MAP"),
        15542 => Some("MAT_DISTORTIONMODIFIERS_FILENAME"),
        15543 => Some("MAT_DISTORTIONMODIFIERS_COMPRESSION"),
        15544 => Some("MAT_DISTORTIONMODIFIERS_COMPRESSION_XBOX360"),
        15560 => Some("MAT_DISTORTION_MAGNITUDE"),
        15561 => Some("MAT_DISTORTION_INVERT"),
        15562 => Some("MAT_DISTORTION_FADE_DISTANCE"),
        15563 => Some("MAT_DISTORTION_FADE_MULTIPLIER"),
        15580 => Some("MAT_ALTERNATE_DECAL_MAP"),
        15581 => Some("MAT_ALTERNATE_DECAL_FILENAME"),
        15582 => Some("MAT_ALTERNATE_DECAL_COMPRESSION"),
        15583 => Some("MAT_ALTERNATE_DECAL_COMPRESSION_XBOX360"),
        15590 => Some("MAT_TATTOO_MASK_MAP"),
        15591 => Some("MAT_TATTOO_MASK_FILENAME"),
        15592 => Some("MAT_TATTOO_MASK_COMPRESSION"),
        15593 => Some("MAT_TATTOO_MASK_COMPRESSION_XBOX360"),
        15594 => Some("MAT_TATTOO_MASK_MAP_ENABLE"),
        15595 => Some("MAT_TATTOO_MASK_TINT_CHANNEL1"),
        15596 => Some("MAT_TATTOO_MASK_TINT_CHANNEL2"),
        15597 => Some("MAT_TATTOO_MASK_TINT_CHANNEL3"),
        15598 => Some("MAT_TATTOO_MASK_TINT_CHANNEL4"),
        15600 => Some("MAT_BROW_STUBBLE_MAP"),
        15601 => Some("MAT_BROW_STUBBLE_FILENAME"),
        15602 => Some("MAT_BROW_STUBBLE_COMPRESSION"),
        15603 => Some("MAT_BROW_STUBBLE_COMPRESSION_XBOX360"),
        15610 => Some("MAT_BROW_STUBBLE_NORMAL_MAP"),
        15611 => Some("MAT_BROW_STUBBLE_NORMAL_FILENAME"),
        15612 => Some("MAT_BROW_STUBBLE_NORMAL_COMPRESSION"),
        15613 => Some("MAT_BROW_STUBBLE_NORMAL_COMPRESSION_XBOX360"),
        15620 => Some("MAT_EMOTIONS_MASK_0_MAP"),
        15621 => Some("MAT_EMOTIONS_MASK_0_FILENAME"),
        15622 => Some("MAT_EMOTIONS_MASK_0_COMPRESSION"),
        15623 => Some("MAT_EMOTIONS_MASK_0_COMPRESSION_XBOX360"),
        15630 => Some("MAT_EMOTIONS_MASK_1_MAP"),
        15631 => Some("MAT_EMOTIONS_MASK_1_FILENAME"),
        15632 => Some("MAT_EMOTIONS_MASK_1_COMPRESSION"),
        15633 => Some("MAT_EMOTIONS_MASK_1_COMPRESSION_XBOX360"),
        15640 => Some("MAT_EMOTIONS_NORMAL_MAP"),
        15641 => Some("MAT_EMOTIONS_NORMAL_FILENAME"),
        15642 => Some("MAT_EMOTIONS_NORMAL_COMPRESSION"),
        15643 => Some("MAT_EMOTIONS_NORMAL_COMPRESSION_XBOX360"),
        15650 => Some("MAT_SCROLL_SPEED_1"),
        15651 => Some("MAT_SCROLL_SPEED_2"),
        15652 => Some("MAT_SCROLL_SPEED_3"),
        15653 => Some("MAT_LAVA_TINT_COLOR"),
        15654 => Some("MAT_LAVA_BRIGHTNESS"),
        15655 => Some("MAT_LAVA_CONTRAST"),
        15656 => Some("MAT_LAVA_NOISE_MAP"),
        16000 => Some("SAVEGAME_CAMPAIGN"),
        16001 => Some("SAVEGAME_AREALIST"),
        16002 => Some("SAVEGAME_PLAYERCHAR"),
        16003 => Some("SAVEGAME_PARTYLIST"),
        16004 => Some("SAVEGAME_VERSION"),
        16005 => Some("SAVEGAME_GAME_STATE"),
        16006 => Some("SAVEGAME_ADDINSLIST"),
        16007 => Some("SAVEGAME_CHEAT_USED"),
        16008 => Some("SAVEGAME_STORYSOFAR"),
        16010 => Some("SAVEGAME_AREA_PLACEABLES"),
        16011 => Some("SAVEGAME_AREA_CREATURES"),
        16012 => Some("SAVEGAME_AREA_TRIGGERS"),
        16013 => Some("SAVEGAME_AREA_AOES"),
        16014 => Some("SAVEGAME_CAMPAIGN_RESOURCE"),
        16015 => Some("SAVEGAME_AREA_WAYPOINTS"),
        16016 => Some("SAVEGAME_AREA_MAP"),
        16017 => Some("SAVEGAME_AREA_STORES"),
        16018 => Some("SAVEGAME_AREA_ROOMS_VIEWED"),
        16019 => Some("SAVEGAME_AREA_SOUNDS"),
        16020 => Some("SAVEGAME_AREA_MIN_CREATURE_IMPORTANCE"),
        16100 => Some("SAVEGAME_AREA_PLACEABLE_STATE"),
        16101 => Some("SAVEGAME_AREA_TRIGGER_GEOMETRY"),
        16102 => Some("SAVEGAME_AREA_PLACEABLE_USEABLE"),
        16103 => Some("SAVEGAME_AREA_TRIGGER_DETECTABLE"),
        16104 => Some("SAVEGAME_AREA_TRIGGER_DISARMABLE"),
        16105 => Some("SAVEGAME_AREA_TRIGGER_DCDETECTCHECK"),
        16106 => Some("SAVEGAME_AREA_TRIGGER_DCDISARMCHECK"),
        16107 => Some("SAVEGAME_AREA_TRIGGER_LAST_DISARMED"),
        16108 => Some("SAVEGAME_AREA_TRIGGER_REVERB_PRESET"),
        16109 => Some("SAVEGAME_AREA_TRIGGER_PRIORITY"),
        16110 => Some("SAVEGAME_AREA_TRIGGER_LOAD_SCREEN"),
        16111 => Some("SAVEGAME_AREA_TRIGGER_SOUNDS"),
        16112 => Some("SAVEGAME_AREA_TRIGGER_TYPE"),
        16113 => Some("SAVEGAME_AREA_TRIGGER_MUSICVOLUME_ENTERSTATE"),
        16114 => Some("SAVEGAME_AREA_TRIGGER_MUSICVOLUME_EXITSTATE"),
        16115 => Some("SAVEGAME_AREA_TRIGGER_MUSICVOLUME_ENTERSTATEDELAY"),
        16116 => Some("SAVEGAME_AREA_TRIGGER_MUSICVOLUME_EXITSTATEDELAY"),
        16150 => Some("SAVEGAME_STORE_MARKDOWN"),
        16151 => Some("SAVEGAME_STORE_MARKUP"),
        16152 => Some("SAVEGAME_STORE_GOLD"),
        16153 => Some("SAVEGAME_STORE_MAXBUYPRICE"),
        16154 => Some("SAVEGAME_STORE_WILLNOTBUY"),
        16155 => Some("SAVEGAME_STORE_WILLONLYBUY"),
        16156 => Some("SAVEGAME_STORE_ITEMLIST"),
        16201 => Some("SAVEGAME_OBJECT_ACTIVE"),
        16203 => Some("SAVEGAME_PARTYMEMBERS"),
        16204 => Some("SAVEGAME_PARTYPOOLMEMBERS"),
        16205 => Some("SAVEGAME_PARTYMEM_CREATURE"),
        16206 => Some("SAVEGAME_PARTYMEM_TEMPLATE"),
        16207 => Some("SAVEGAME_PARTYCREATURES"),
        16208 => Some("SAVEGAME_PLAYERCHAR_CHAR"),
        16209 => Some("SAVEGAME_CREATURE_STATS"),
        16210 => Some("SAVEGAME_BACKPACK"),
        16211 => Some("SAVEGAME_PLOTITEMS"),
        16212 => Some("SAVEGAME_MONEY"),
        16213 => Some("SAVEGAME_QUICKITEMS"),
        16214 => Some("SAVEGAME_EQUIPMENT"),
        16215 => Some("SAVEGAME_EQUIPMENTSET"),
        16216 => Some("SAVEGAME_EQUIPMENTSET_SLOT"),
        16217 => Some("SAVEGAME_EQUIPMENTSET_OBJECT"),
        16218 => Some("SAVEGAME_EQUIPMENT_ACTIVESET"),
        16219 => Some("SAVEGAME_EQUIPMENT_ITEMS"),
        16220 => Some("SAVEGAME_OBJECT_IMMORTAL"),
        16221 => Some("SAVEGAME_OBJECT_EVENTSCRIPT"),
        16222 => Some("SAVEGAME_OBJECT_TAG"),
        16223 => Some("SAVEGAME_ITEMS"),
        16224 => Some("SAVEGAME_ITEM_DROPPABLE"),
        16225 => Some("SAVEGAME_ITEM_DAMAGED"),
        16226 => Some("SAVEGAME_MAX_ITEMS"),
        16227 => Some("SAVEGAME_CRAFTING_RECIPE_LIST"),
        16228 => Some("SAVEGAME_ITEM_IRREMOVABLE"),
        16229 => Some("SAVEGAME_ITEM_INDESTRUCTIBLE"),
        16230 => Some("SAVEGAME_ITEM_MATERIALTYPE"),
        16231 => Some("SAVEGAME_ITEM_STEALABLE"),
        16232 => Some("SAVEGAME_ITEM_INFINITE"),
        16233 => Some("SAVEGAME_ITEM_CURRENT_VFX_PROPERTY_ID"),
        16234 => Some("SAVEGAME_ITEM_CURRENT_VFX_PROPERTY_POWER"),
        16250 => Some("SAVEGAME_OBJECT_PLOT"),
        16251 => Some("SAVEGAME_OBJECT_HEALTH"),
        16252 => Some("SAVEGAME_OBJECT_MAX_HEALTH"),
        16253 => Some("SAVEGAME_OBJECT_RANK"),
        16254 => Some("SAVEGAME_OBJECT_TREASURE_GROUP"),
        16255 => Some("SAVEGAME_OBJECT_NAME"),
        16256 => Some("SAVEGAME_OBJECT_LOOPING_ANIMATION"),
        16257 => Some("SAVEGAME_OBJECT_LOOTABLE_CREATURE_APPEARANCETYPE"),
        16258 => Some("SAVEGAME_OBJECT_PICKLOCK"),
        16259 => Some("SAVEGAME_OBJECT_TRAP_DETECTED"),
        16260 => Some("SAVEGAME_OBJECT_DCDETECTCHECK"),
        16261 => Some("SAVEGAME_OBJECT_DCDISARMCHECK"),
        16262 => Some("SAVEGAME_OBJECT_INTERACTION_RADIUS"),
        16263 => Some("SAVEGAME_OBJECT_IMPORTANCE"),
        16270 => Some("SAVEGAME_SELECTED_CHARACTER"),
        16274 => Some("SAVEGAME_PARTY_PICKER_GUI_STATUS"),
        16275 => Some("SAVEGAME_PARTY_APPROVAL_LIST"),
        16276 => Some("SAVEGAME_PARTY_APPROVAL_ID"),
        16277 => Some("SAVEGAME_PARTY_APPROVAL_LEVEL"),
        16278 => Some("SAVEGAME_PARTY_LEADER"),
        16279 => Some("SAVEGAME_NONPARTYMEMBERS"),
        16280 => Some("SAVEGAME_PARTY_MEMBER_SUBSTATE"),
        16281 => Some("SAVEGAME_PARTY_MEMBER_LOCKED"),
        16282 => Some("SAVEGAME_PARTY_MEMBER_FOLLOW"),
        16284 => Some("SAVEGAME_PARTY_ITEM_STORAGE_ITEM"),
        16285 => Some("SAVEGAME_PARTY_ITEM_STORAGE_OWNER"),
        16286 => Some("SAVEGAME_PARTY_ITEM_STORAGE_SLOT"),
        16287 => Some("SAVEGAME_PARTY_ITEM_STORAGE_WEAPONSET"),
        16288 => Some("SAVEGAME_PARTY_ITEM_STORAGE_LIST"),
        16289 => Some("SAVEGAME_PARTY_NEW_ITEM_ID"),
        16290 => Some("SAVEGAME_PARTY_NEW_ITEM_LIST"),
        16291 => Some("SAVEGAME_PARTY_AUTO_LEVEL_DEFAULT"),
        16292 => Some("SAVEGAME_PARTY_QUICKBAR_LOCKED"),
        16293 => Some("SAVEGAME_PARTY_HOLD_POSITIONS"),
        16294 => Some("SAVEGAME_PARTY_RUN_IN_DRIVE_MODE"),
        16295 => Some("SAVEGAME_PLAYER_MAP_ZOOM"),
        16296 => Some("SAVEGAME_PLAYER_MAP_LEGEND"),
        16297 => Some("SAVEGAME_PARTY_APPROVAL_DESC"),
        16298 => Some("SAVEGAME_PLAYER_TIME_PLAYED"),
        16299 => Some("SAVEGAME_PARTY_BACKPACK_SORT"),
        16300 => Some("SAVEGAME_STATPROPERTY_BASE"),
        16301 => Some("SAVEGAME_STATPROPERTY_MODIFIER"),
        16302 => Some("SAVEGAME_STATPROPERTY_CURRENT"),
        16303 => Some("SAVEGAME_STATPROPERTY_COMREGEN"),
        16304 => Some("SAVEGAME_STATPROPERTY_REGEN"),
        16305 => Some("SAVEGAME_SPELLLIST"),
        16306 => Some("SAVEGAME_TALENTLIST"),
        16307 => Some("SAVEGAME_SKILLLIST"),
        16308 => Some("SAVEGAME_QUICKSLOTS"),
        16309 => Some("SAVEGAME_ABILITYLIST"),
        16310 => Some("SAVEGAME_QBAR_EXPANSION_VALUE"),
        16311 => Some("SAVEGAME_QUICKSLOT_ABILITY"),
        16312 => Some("SAVEGAME_QUICKSLOT_ITEMTAG"),
        16313 => Some("SAVEGAME_QUICKSLOTS1"),
        16314 => Some("SAVEGAME_QUICKSLOTS2"),
        16315 => Some("SAVEGAME_QUICKSLOTS3"),
        16316 => Some("SAVEGAME_QUICKSLOTS4"),
        16317 => Some("SAVEGAME_CURENTQBAR"),
        16318 => Some("SAVEGAME_LOCKQBAR"),
        16319 => Some("SAVEGAME_QUICKSLOT_TEMPLATE"),
        16320 => Some("SAVEGAME_APPEARANCE"),
        16321 => Some("SAVEGAME_APPEARANCE_TYPE"),
        16322 => Some("SAVEGAME_APPEARANCE_GENDER"),
        16324 => Some("SAVEGAME_APPEARANCE_GORE"),
        16325 => Some("SAVEGAME_APPEARANCE_DECAPITATED"),
        16326 => Some("SAVEGAME_APPEARANCE_ITEM_HERALDRY_VARIATION"),
        16327 => Some("SAVEGAME_APPEARANCE_ORIGINAL_TYPE"),
        16328 => Some("SAVEGAME_APPEARANCE_MORPH_NAME"),
        16329 => Some("SAVEGAME_AUTOLEVELUP"),
        16331 => Some("SAVEGAME_QUICKSLOT_NUMBER"),
        16332 => Some("SAVEGAME_PLAYER_PORTRAIT_PITCH"),
        16333 => Some("SAVEGAME_PLAYER_PORTRAIT_YAW"),
        16334 => Some("SAVEGAME_PLAYER_PORTRAIT_TINT"),
        16335 => Some("SAVEGAME_PLAYER_PORTRAIT_EXPRESSION"),
        16336 => Some("SAVEGAME_PLAYER_PORTRAIT_DISTANCE"),
        16337 => Some("SAVEGAME_PLAYER_PORTRAIT_POSITIONH"),
        16338 => Some("SAVEGAME_PLAYER_PORTRAIT_POSITIONV"),
        16350 => Some("SAVEGAME_STATLIST"),
        16351 => Some("SAVEGAME_HEROIC_STATLIST"),
        16352 => Some("SAVEGAME_HEROIC_PARTY_STATLIST"),
        16353 => Some("SAVEGAME_STATPROPERTY_INDEX"),
        16400 => Some("SAVEGAME_PLOT_MANAGER"),
        16401 => Some("SAVEGAME_PLOT_LIST"),
        16402 => Some("SAVEGAME_PLOT_GUID"),
        16403 => Some("SAVEGAME_PLOT_FLAGS_1"),
        16404 => Some("SAVEGAME_PLOT_FLAGS_2"),
        16405 => Some("SAVEGAME_PLOT_FLAGS_3"),
        16406 => Some("SAVEGAME_PLOT_FLAGS_4"),
        16420 => Some("SAVEGAME_ADDIN_UID"),
        16421 => Some("SAVEGAME_ADDIN_ENUS"),
        16422 => Some("SAVEGAME_ADDIN_FRFR"),
        16423 => Some("SAVEGAME_ADDIN_ITIT"),
        16424 => Some("SAVEGAME_ADDIN_DEDE"),
        16425 => Some("SAVEGAME_ADDIN_ESES"),
        16426 => Some("SAVEGAME_ADDIN_PLPL"),
        16427 => Some("SAVEGAME_ADDIN_RURU"),
        16428 => Some("SAVEGAME_ADDIN_PSEUDO"),
        16429 => Some("SAVEGAME_ADDIN_CSCZ"),
        16430 => Some("SAVEGAME_ADDIN_HUHU"),
        16450 => Some("SAVEGAME_GROUP_LIST"),
        16451 => Some("SAVEGAME_GROUP_ID"),
        16452 => Some("SAVEGAME_GROUP_HOSTILES"),
        16453 => Some("SAVEGAME_TEAM_ID"),
        16454 => Some("SAVEGAME_CREATURE_STEALTH"),
        16455 => Some("SAVEGAME_IS_PLOT_GIVER"),
        16456 => Some("SAVEGAME_CAN_LEVELUP"),
        16457 => Some("SAVEGAME_CREATURE_TRACKABLE"),
        16458 => Some("SAVEGAME_CREATURE_CONTROLLABLE"),
        16459 => Some("SAVEGAME_CREATURE_INTERACTIVE"),
        16460 => Some("SAVEGAME_CREATURE_RACE"),
        16461 => Some("SAVEGAME_CREATURE_PACKAGE"),
        16462 => Some("SAVEGAME_CREATURE_PACKAGE_AI"),
        16463 => Some("SAVEGAME_CREATURE_CANCHANGEEQUIPMENT"),
        16464 => Some("SAVEGAME_CREATURE_CLASS_RANK_LIST"),
        16465 => Some("SAVEGAME_CREATURE_CLASS_ID"),
        16466 => Some("SAVEGAME_CREATURE_CLASS_RANK"),
        16467 => Some("SAVEGAME_CREATURE_IS_GHOST"),
        16468 => Some("SAVEGAME_CREATURE_MODAL_ABILITY_LIST"),
        16469 => Some("SAVEGAME_CREATURE_SHOW_AS_ALLY_ON_MAP"),
        16470 => Some("SAVEGAME_CREATURE_IS_STATUE"),
        16471 => Some("SAVEGAME_CREATURE_MINIMIZED_SKILL_HEADER_LIST"),
        16472 => Some("SAVEGAME_CREATURE_MINIMIZED_TALENT_HEADER_LIST"),
        16473 => Some("SAVEGAME_CREATURE_ABILITY_HEADER_ID"),
        16474 => Some("SAVEGAME_CREATURE_ITEMS_SCALED"),
        16475 => Some("SAVEGAME_CREATURE_HEATBEAT_INTERVAL"),
        16476 => Some("SAVEGAME_CREATURE_ROAM_RADIUS"),
        16477 => Some("SAVEGAME_CREATURE_ROAM_CENTER"),
        16478 => Some("SAVEGAME_CREATURE_POOL_NAME"),
        16479 => Some("SAVEGAME_CREATURE_POOL_AVAILABLE"),
        16480 => Some("SAVEGAME_CREATURE_NOPERMDEATH"),
        16481 => Some("SAVEGAME_CREATURE_TIMESINCEDEATH"),
        16499 => Some("SAVEGAME_CREATURE_TIMEBEFOREDECAY"),
        16500 => Some("SAVEGAME_WORLDDATABASE"),
        16501 => Some("SAVEGAME_WORLDDB_IDGROUP"),
        16502 => Some("SAVEGAME_WORLDDB_LASTID"),
        16503 => Some("SAVEGAME_PARTY_SEEN_LINES"),
        16504 => Some("SAVEGAME_JOURNAL"),
        16505 => Some("SAVEGAME_JOURNAL_ACTIVE_LIST"),
        16506 => Some("SAVEGAME_JOURNAL_COMPLETE_LIST"),
        16507 => Some("SAVEGAME_JOURNAL_TITLE"),
        16508 => Some("SAVEGAME_JOURNAL_TEXT"),
        16509 => Some("SAVEGAME_JOURNAL_PARENT_PLOT"),
        16510 => Some("SAVEGAME_JOURNAL_RESREF"),
        16511 => Some("SAVEGAME_JOURNAL_STORY_TEXT"),
        16512 => Some("SAVEGAME_JOURNAL_AREA_TAG"),
        16513 => Some("SAVEGAME_JOURNAL_PLOT_DESTINATION_LIST"),
        16514 => Some("SAVEGAME_JOURNAL_PLOT_DESTINATION_TAG"),
        16515 => Some("SAVEGAME_JOURNAL_PLOT_DESTINATION_GUID_LIST"),
        16516 => Some("SAVEGAME_JOURNAL_CONVERSATION_LIST"),
        16517 => Some("SAVEGAME_JOURNAL_CONVERSATION_LINE_LIST"),
        16518 => Some("SAVEGAME_JOURNAL_CONVERSATION_LINE_SPEAKER"),
        16519 => Some("SAVEGAME_JOURNAL_CONVERSATION_LINE_TEXT"),
        16520 => Some("SAVEGAME_JOURNAL_CONVERSATION_LINE_REPLY"),
        16521 => Some("SAVEGAME_JOURNAL_UNREAD_CODEX_LIST"),
        16522 => Some("SAVEGAME_JOURNAL_ORPHAN_LIST"),
        16523 => Some("SAVEGAME_JOURNAL_QUEST_COMPLETED"),
        16524 => Some("SAVEGAME_JOURNAL_QUEST_GROUP"),
        16525 => Some("SAVEGAME_JOURNAL_GROUP_LIST"),
        16526 => Some("SAVEGAME_JOURNAL_GROUP_RESREF"),
        16527 => Some("SAVEGAME_JOURNAL_GROUP_OPEN_IN_CURRENT"),
        16528 => Some("SAVEGAME_JOURNAL_GROUP_OPEN_IN_COMPLETED"),
        16529 => Some("SAVEGAME_JOURNAL_GROUP_PRIORITY"),
        16530 => Some("SAVEGAME_AMBIENTDIALOG_LIST"),
        16531 => Some("SAVEGAME_AMBIENTDIALOG_OWNER"),
        16532 => Some("SAVEGAME_AMBIENTDIALOG_SPEAKER"),
        16533 => Some("SAVEGAME_AMBIENTDIALOG_RESREF"),
        16534 => Some("SAVEGAME_AMBIENTDIALOG_LINE"),
        16540 => Some("SAVEGAME_JOURNAL_QUEST_UPDATED"),
        16541 => Some("SAVEGAME_JOURNAL_OFFER_ID"),
        16600 => Some("SAVEGAME_BODYBAG_ID"),
        16601 => Some("SAVEGAME_ISBODYBAG"),
        16602 => Some("SAVEGAME_LOOTABLE_OBJECT_ID"),
        16603 => Some("SAVEGAME_AOE_ID"),
        16604 => Some("SAVEGAME_AOE_SHAPE"),
        16605 => Some("SAVEGAME_AOE_RADIUS"),
        16606 => Some("SAVEGAME_AOE_WIDTH"),
        16607 => Some("SAVEGAME_AOE_LENGTH"),
        16608 => Some("SAVEGAME_AOE_CREATOR"),
        16609 => Some("SAVEGAME_AOE_DURATION"),
        16610 => Some("SAVEGAME_AOE_DURATION_TYPE"),
        16611 => Some("SAVEGAME_AOE_LINKED"),
        16612 => Some("SAVEGAME_CREATURE_RANK"),
        16613 => Some("SAVEGAME_EFFECT_ID"),
        16614 => Some("SAVEGAME_EFFECT_TYPE"),
        16615 => Some("SAVEGAME_EFFECT_DURATION_TYPE"),
        16616 => Some("SAVEGAME_EFFECT_DURATION"),
        16617 => Some("SAVEGAME_EFFECT_SUBTYPE"),
        16618 => Some("SAVEGAME_EFFECT_TIMEINDEX"),
        16619 => Some("SAVEGAME_EFFECT_ANIMATION"),
        16620 => Some("SAVEGAME_EFFECT_PRIORITY"),
        16621 => Some("SAVEGAME_EFFECT_CREATOR"),
        16622 => Some("SAVEGAME_EFFECT_ABILITY_ID"),
        16623 => Some("SAVEGAME_EFFECT_LIST"),
        16624 => Some("SAVEGAME_EFFECT_ENGINE_DATA"),
        16625 => Some("SAVEGAME_EFFECT_RESOURCE2"),
        16626 => Some("SAVEGAME_EFFECT_STARTINGID"),
        16627 => Some("SAVEGAME_EFFECT_FLAGS"),
        16630 => Some("SAVEGAME_EVENT_QUEUE"),
        16631 => Some("SAVEGAME_EVENT_DAY"),
        16632 => Some("SAVEGAME_EVENT_TIME"),
        16633 => Some("SAVEGAME_EVENT_CALLER_ID"),
        16634 => Some("SAVEGAME_EVENT_TARGET_ID"),
        16635 => Some("SAVEGAME_EVENT_ID"),
        16636 => Some("SAVEGAME_AI_MASTER"),
        16640 => Some("SAVEGAME_DATAARRAY"),
        16641 => Some("SAVEGAME_DATAARRAY_INT"),
        16642 => Some("SAVEGAME_DATAARRAY_FLOAT"),
        16643 => Some("SAVEGAME_DATAARRAY_BOOL"),
        16644 => Some("SAVEGAME_DATAARRAY_OID"),
        16645 => Some("SAVEGAME_DATAARRAY_STRING"),
        16646 => Some("SAVEGAME_DATAARRAY_VECTOR"),
        16647 => Some("SAVEGAME_DATAARRAY_QUATERNION"),
        16650 => Some("SAVEGAME_EVENT_SCRIPT"),
        16651 => Some("SAVEGAME_EVENT_SIMPLE_VALUE"),
        16670 => Some("SAVEGAME_SCRIPT_EVENT_TYPE"),
        16671 => Some("SAVEGAME_SCRIPT_EVENT_CREATOR"),
        16672 => Some("SAVEGAME_SCRIPT_EVENT_TARGET"),
        16673 => Some("SAVEGAME_SCRIPT_EVENT_DATA"),
        16674 => Some("SAVEGAME_SCRIPT_EVENT_SCRIPT_NAME"),
        16675 => Some("SAVEGAME_SCRIPT_EVENT_RESOURCE_LIST"),
        16700 => Some("SAVEGAME_WORLD_TIMER"),
        16701 => Some("SAVEGAME_WORLD_TIMER_DAY"),
        16702 => Some("SAVEGAME_WORLD_TIMER_TIME"),
        16710 => Some("SAVEGAME_WAYPOINT_MAPNOTE"),
        16711 => Some("SAVEGAME_WAYPOINT_MAPNOTE_ENABLED"),
        16712 => Some("SAVEGAME_WAYPOINT_MAPNOTE_TEXT"),
        16713 => Some("SAVEGAME_WAYPOINT_MAPNOTE_TYPE"),
        16714 => Some("SAVEGAME_WAYPOINT_MAPNOTE_LOC_TEXT"),
        16720 => Some("SAVEGAME_CURRENT_COMMAND"),
        16721 => Some("SAVEGAME_COMMAND_LIST"),
        16722 => Some("SAVEGAME_COMMAND_COMMANDID"),
        16723 => Some("SAVEGAME_COMMAND_ID"),
        16724 => Some("SAVEGAME_COMMAND_STATIC"),
        16725 => Some("SAVEGAME_COMMAND_DATA"),
        16726 => Some("SAVEGAME_COMMAND_PLAYERISSUED"),
        16730 => Some("SAVEGAME_SUBACTION_LIST"),
        16731 => Some("SAVEGAME_SUBACTION_ID"),
        16732 => Some("SAVEGAME_SUBACTION_CORE_SUBACTION"),
        16733 => Some("SAVEGAME_SUBACTION_CORE_INTERRUPTABLE"),
        16734 => Some("SAVEGAME_SUBACTION_TIME_INDEX"),
        16735 => Some("SAVEGAME_SUBACTION_LAST_TIME_INDEX"),
        16736 => Some("SAVEGAME_SUBACTION_LENGTH"),
        16737 => Some("SAVENAME_SUBACTION_START_TIME"),
        16738 => Some("SAVEGAME_SUBACTION_DATA"),
        16740 => Some("SAVEGAME_CURRENT_ACTION_QUEUE"),
        16750 => Some("SAVEGAME_AOE_ABILITY_ID"),
        16751 => Some("SAVEGAME_AOE_FLAGS"),
        16752 => Some("SAVEGAME_AOE_STATIONARY"),
        16770 => Some("SAVEGAME_BUILD_NUMBER"),
        16771 => Some("SAVEGAME_SAVE_VERSION_INTERNAL"),
        16780 => Some("SAVEGAME_WORLDMAP"),
        16781 => Some("SAVEGAME_WORLDMAP_PRIMARYMAP"),
        16782 => Some("SAVEGAME_WORLDMAP_SECONDARYMAP"),
        16783 => Some("SAVEGAME_WORLDMAP_MAPLIST"),
        16784 => Some("SAVEGAME_WORLDMAP_MAP_TAG"),
        16785 => Some("SAVEGAME_WORLDMAP_MAP_PLAYERLOC"),
        16786 => Some("SAVEGAME_WORLDMAP_MAP_PINLIST"),
        16787 => Some("SAVEGAME_WORLDMAP_MAPPIN_TAG"),
        16788 => Some("SAVEGAME_WORLDMAP_MAPPIN_STATE"),
        16789 => Some("SAVEGAME_WORLDMAP_MAPPIN_RECENTLY_ACTIVATED"),
        16790 => Some("SAVEGAME_WORLDMAP_GUI_STATUS"),
        16791 => Some("SAVEGAME_WORLDMAP_LAST_PIN_CLICKED"),
        16792 => Some("SAVEGAME_WORLDMAP_MAPPIN_ACTIVATED_PREVIOUSLY"),
        16793 => Some("SAVEGAME_WORLDMAP_MAPPIN_LAST_STATE"),
        16794 => Some("SAVEGAME_WORLDMAP_TRAVELPOINT_POSX"),
        16795 => Some("SAVEGAME_WORLDMAP_TRAVELPOINT_POSY"),
        16796 => Some("SAVEGAME_WORLDMAP_MAP_TRAVELPATH_BEFORE"),
        16797 => Some("SAVEGAME_WORLDMAP_MAP_TRAVELPATH_AFTER"),
        16798 => Some("SAVEGAME_WORLDMAP_MAPPIN_NAME"),
        16800 => Some("SAVEGAME_META_AREANAME"),
        16801 => Some("SAVEGAME_META_TIMEPLAYED"),
        16802 => Some("SAVEGAME_META_LEVEL"),
        16803 => Some("SAVEGAME_META_CLASS"),
        16804 => Some("SAVEGAME_META_GENDER"),
        16805 => Some("SAVEGAME_META_RACE"),
        16806 => Some("SAVEGAME_META_BACKGROUND"),
        16807 => Some("SAVEGAME_META_NAME"),
        16808 => Some("SAVEGAME_META_SAVENAME"),
        16818 => Some("SAVEGAME_TACTICENTRY_TARGET_OBJECT_ID"),
        16819 => Some("SAVEGAME_TACTICENTRY_CONDITION_OBJECT_ID"),
        16820 => Some("SAVEGAME_PARTY_TACTICS_ITEM_ABILITIES"),
        16821 => Some("SAVEGAME_TACTICS_HAS_TABLE"),
        16822 => Some("SAVEGAME_TACTICS_TABLE"),
        16823 => Some("SAVEGAME_TACTICS_ENABLED"),
        16824 => Some("SAVEGAME_TACTICS_LIST"),
        16825 => Some("SAVEGAME_TACTICENTRY_ENABLED"),
        16826 => Some("SAVEGAME_TACTICENTRY_TARGET"),
        16827 => Some("SAVEGAME_TACTICENTRY_CONDITION"),
        16828 => Some("SAVEGAME_TACTICENTRY_COMMAND"),
        16829 => Some("SAVEGAME_TACTICENTRY_COMMANDPARAM"),
        16830 => Some("SAVEGAME_TACTICENTRY_TARGETTAG"),
        16831 => Some("SAVEGAME_TACTICENTRY_CONDITIONTAG"),
        16832 => Some("SAVEGAME_TACTICS_DIRTY"),
        16833 => Some("SAVEGAME_TACTICS_PRESETTYPE"),
        16834 => Some("SAVEGAME_TACTICS_PRESETINDEX"),
        16835 => Some("SAVEGAME_TACTICS_PRESETLIST"),
        16836 => Some("SAVEGAME_TACTICS_CUSTOMLIST"),
        16837 => Some("SAVEGAME_TACTICENTRY_COMMANDITEMTAG"),
        16838 => Some("SAVEGAME_TACTICENTRY_COMMANDITEMRESREF"),
        16840 => Some("SAVEGAME_PLOTACTIONS"),
        16841 => Some("SAVEGAME_PLOTACTIONS_ENABLED"),
        16842 => Some("SAVEGAME_PLOTACTIONS_CURRENTSET"),
        16843 => Some("SAVEGAME_PLOTACTIONS_LIST"),
        16844 => Some("SAVEGAME_PLOTACTION_ID"),
        16845 => Some("SAVEGAME_PLOTACTION_STATE"),
        16846 => Some("SAVEGAME_PLOTACTION_COUNT"),
        16847 => Some("SAVEGAME_PLOTACTION_UPDATED"),
        16900 => Some("SAVEGAME_SOUND_TAG"),
        16901 => Some("SAVEGAME_SOUND_ACTIVE"),
        16902 => Some("SAVEGAME_SOUND_NAME"),
        16903 => Some("SAVEGAME_SOUND_XPOSITION"),
        16904 => Some("SAVEGAME_SOUND_YPOSITION"),
        16905 => Some("SAVEGAME_SOUND_ZPOSITION"),
        16906 => Some("SAVEGAME_SOUND_XORIENTATION"),
        16907 => Some("SAVEGAME_SOUND_YORIENTATION"),
        16908 => Some("SAVEGAME_SOUND_ZORIENTATION"),
        16909 => Some("SAVEGAME_SOUND_WORIENTATION"),
        16910 => Some("SAVEGAME_SOUND_VOLUME"),
        16911 => Some("SAVEGAME_SOUND_PITCH"),
        16912 => Some("SAVEGAME_SOUND_FADEIN"),
        16913 => Some("SAVEGAME_SOUND_FADEOUT"),
        16914 => Some("SAVEGAME_SOUND_MAXDISTANCEMULT"),
        16915 => Some("SAVEGAME_SOUND_CONEINSIDE"),
        16916 => Some("SAVEGAME_SOUND_CONEOUTSIDE"),
        16917 => Some("SAVEGAME_SOUND_CONEVOLUME"),
        16918 => Some("SAVEGAME_SOUND_PRIORITY"),
        16919 => Some("SAVEGAME_SOUND_OCCLUDABLE"),
        16950 => Some("SAVEGAME_PLAYER_MORPH"),
        16951 => Some("SAVEGAME_PLAYER_SOUNDSET"),
        16952 => Some("SAVEGAME_DEFAULT_SOUNDSET"),
        16960 => Some("SAVEGAME_ADDIN_NAME"),
        16970 => Some("SAVEGAME_STORYSOFAR_EVENTLIST"),
        16971 => Some("SAVEGAME_STORYSOFAR_EVENTID"),
        16972 => Some("SAVEGAME_STORYSOFAR_GAMETIME"),
        16973 => Some("SAVEGAME_STORYSOFAR_UTC"),
        16974 => Some("SAVEGAME_STORYSOFAR_SCREENSHOT"),
        16975 => Some("SAVEGAME_STORYSOFAR_LEVELUPLIST"),
        16976 => Some("SAVEGAME_STORYSOFAR_AREA"),
        16977 => Some("SAVEGAME_STORYSOFAR_LEVEL"),
        16978 => Some("SAVEGAME_STORYSOFAR_MONEY"),
        16979 => Some("SAVEGAME_STORYSOFAR_CURRENT_HEATLH"),
        16980 => Some("SAVEGAME_STORYSOFAR_TOTAL_HEATLH"),
        16981 => Some("SAVEGAME_STORYSOFAR_CURRENT_STAMINA"),
        16982 => Some("SAVEGAME_STORYSOFAR_TOTAL_STAMINA"),
        16983 => Some("SAVEGAME_STORYSOFAR_CURRENT_XP"),
        16984 => Some("SAVEGAME_STORYSOFAR_SPELL_LIST"),
        16985 => Some("SAVEGAME_STORYSOFAR_TALENT_LIST"),
        16986 => Some("SAVEGAME_STORYSOFAR_SKILL_LIST"),
        16987 => Some("SAVEGAME_STORYSOFAR_ATTRIBUTE_LIST"),
        16988 => Some("SAVEGAME_STORYSOFAR_ATTRIBUTE_BASE"),
        16989 => Some("SAVEGAME_STORYSOFAR_ATTRIBUTE_MODIFIER"),
        16990 => Some("SAVEGAME_STORYSOFAR_EQUIPMENT_LIST"),
        16991 => Some("SAVEGAME_STORYSOFAR_EQUIPMENT_SLOTID"),
        16992 => Some("SAVEGAME_STORYSOFAR_EQUIPMENT_RESREF"),
        16993 => Some("SAVEGAME_STORYSOFAR_EQUIPMENT_STACKSIZE"),
        16994 => Some("SAVEGAME_STORYSOFAR_ITEM_PROPERTY"),
        16995 => Some("SAVEGAME_STORYSOFAR_ITEM_POWER"),
        16996 => Some("SAVEGAME_STORYSOFAR_ITEM_DATA"),
        17000 => Some("SCRIPTVARTABLE"),
        17001 => Some("SCRIPTVARTABLE_NAME"),
        17002 => Some("SCRIPTVARTABLE_TYPE"),
        17003 => Some("SCRIPTVARTABLE_VALUE"),
        17100 => Some("CAMPAIGN_CIF_ENTRY_AREA_LIST"),
        17101 => Some("CAMPAIGN_CIF_ENTRY_AREA"),
        17102 => Some("CAMPAIGN_CIF_ENTRY_POSITION"),
        17103 => Some("CAMPAIGN_CIF_ENTRY_ORIENTATION"),
        17104 => Some("CAMPAIGN_CIF_ENTRY_SCRIPT"),
        17105 => Some("CAMPAIGN_CIF_ENTRY_CLIENT_SCRIPT"),
        17106 => Some("CAMPAIGN_CIF_DISPLAY_NAME_EN_US"),
        17107 => Some("CAMPAIGN_CIF_DISPLAY_NAME_FR_FR"),
        17108 => Some("CAMPAIGN_CIF_DISPLAY_NAME_DE_DE"),
        17109 => Some("CAMPAIGN_CIF_DISPLAY_NAME_PL_PL"),
        17110 => Some("CAMPAIGN_CIF_DISPLAY_NAME_RU_RU"),
        17111 => Some("CAMPAIGN_CIF_DISPLAY_NAME_IT_IT"),
        17112 => Some("CAMPAIGN_CIF_DISPLAY_NAME_ES_ES"),
        17113 => Some("CAMPAIGN_CIF_DISPLAY_NAME_HU_HU"),
        17114 => Some("CAMPAIGN_CIF_DISPLAY_NAME_CS_CZ"),
        17115 => Some("CAMPAIGN_CIF_DESCRIPTION_EN_US"),
        17116 => Some("CAMPAIGN_CIF_DESCRIPTION_FR_FR"),
        17117 => Some("CAMPAIGN_CIF_DESCRIPTION_DE_DE"),
        17118 => Some("CAMPAIGN_CIF_DESCRIPTION_PL_PL"),
        17119 => Some("CAMPAIGN_CIF_DESCRIPTION_RU_RU"),
        17120 => Some("CAMPAIGN_CIF_DESCRIPTION_IT_IT"),
        17121 => Some("CAMPAIGN_CIF_DESCRIPTION_ES_ES"),
        17122 => Some("CAMPAIGN_CIF_DESCRIPTION_HU_HU"),
        17123 => Some("CAMPAIGN_CIF_DESCRIPTION_CS_CZ"),
        17124 => Some("CAMPAIGN_CIF_PACKAGES_LIST"),
        19000 => Some("TALK_BUCKET_LIST"),
        19001 => Some("TALK_STRING_LIST"),
        19002 => Some("TALK_STRING_ID"),
        19003 => Some("TALK_STRING"),
        20000 => Some("PLACEABLE_STATES_LIST"),
        21000 => Some("VFX_CHILD_LIST"),
        21001 => Some("VFX_OBJECT_ID"),
        21002 => Some("VFX_EMITTER_INITIALROTATIONRANGE"),
        21004 => Some("VFX_ROOT"),
        21005 => Some("VFX_EMITTER_MESH_PARTICLE_ROLL_AXIS"),
        21006 => Some("VFX_TYPE"),
        21007 => Some("VFX_OBJECT_VISIBLE"),
        21008 => Some("VFX_EMITTER_MESH_PARTICLE_UP_AXIS"),
        21009 => Some("VFX_KEYFRAME"),
        21010 => Some("VFX_VALUE"),
        21011 => Some("VFX_EMITTER_NAME"),
        21012 => Some("VFX_EMITTER_TYPE"),
        21013 => Some("VFX_EMITTER_ORIENTATIONBEHAVIOUR"),
        21014 => Some("VFX_EMITTER_UPDATEONLYWHENVISIBLE"),
        21015 => Some("VFX_EMITTER_LINKPARTICLESTOGETHER"),
        21016 => Some("VFX_EMITTER_MATERIALLIBRARY"),
        21017 => Some("VFX_EMITTER_MATERIALOBJECT"),
        21018 => Some("VFX_EMITTER_BIRTHRATE"),
        21019 => Some("VFX_EMITTER_BIRTHRATERANGE"),
        21020 => Some("VFX_EMITTER_BIRTHRATEINPARTICLESPERMETER"),
        21021 => Some("VFX_EMITTER_INITIALSPEED"),
        21022 => Some("VFX_EMITTER_INITIALSPEEDRANGE"),
        21023 => Some("VFX_EMITTER_ACCELERATION"),
        21024 => Some("VFX_EMITTER_GRAVITYMULTIPLIER"),
        21025 => Some("VFX_EMITTER_LIFE"),
        21026 => Some("VFX_EMITTER_LIFERANGE"),
        21027 => Some("VFX_EMITTER_SCALERANGE"),
        21028 => Some("VFX_EMITTER_SPREADX"),
        21029 => Some("VFX_EMITTER_SPREADY"),
        21030 => Some("VFX_EMITTER_INITIALROTATIONSPEED"),
        21031 => Some("VFX_EMITTER_INITIALROTATIONSPEEDRANGE"),
        21032 => Some("VFX_EMITTER_ROTATIONALACCELERATION"),
        21033 => Some("VFX_EMITTER_RANDOMINITIALROTATION"),
        21034 => Some("VFX_EMITTER_PARTICLEINHERITANCE"),
        21035 => Some("VFX_EMITTER_INHERITVELOCITYINSTEADOFPOSITION"),
        21036 => Some("VFX_EMITTER_PARTICLESAFFECTEDBYWIND"),
        21037 => Some("VFX_EMITTER_ENABLEPARTICLECOLLISIONS"),
        21038 => Some("VFX_EMITTER_PHYSICSOBJECTSPAWN"),
        21039 => Some("VFX_EMITTER_PHYSICSEMITTER"),
        21040 => Some("VFX_EMITTER_MOVEMENTSPREADX"),
        21041 => Some("VFX_EMITTER_MOVEMENTSPREADY"),
        21042 => Some("VFX_EMITTER_MOVEMENTSPREADUPDATEDELAY"),
        21043 => Some("VFX_EMITTER_TARGETNAME"),
        21044 => Some("VFX_EMITTER_TARGETATTRACTION"),
        21045 => Some("VFX_EMITTER_TARGETRADIUS"),
        21046 => Some("VFX_EMITTER_SPAWNDIRECTIONTRACKSTARGET"),
        21047 => Some("VFX_EMITTER_KILLPARTICLEWHENTARGETHIT"),
        21048 => Some("VFX_EMITTER_PARTICLESFOLLOWPATH"),
        21049 => Some("VFX_EMITTER_FLIPBOOK_TYPE"),
        21050 => Some("VFX_EMITTER_FLIPBOOK_FRAMES_PER_SECOND"),
        21051 => Some("VFX_EMITTER_FLIPBOOK_ROWS"),
        21052 => Some("VFX_EMITTER_FLIPBOOK_COLUMNS"),
        21053 => Some("VFX_EMITTER_FLIPBOOK_RANDOM_START_FRAME"),
        21054 => Some("VFX_EMITTER_ALPHAMULTIPLIER"),
        21055 => Some("VFX_EMITTER_COLORMULTIPLIER"),
        21056 => Some("VFX_EMITTER_SCALEMULTIPLIER"),
        21057 => Some("VFX_EMITTER_INFINITELIFE"),
        21058 => Some("VFX_EMITTER_CHUNKY_MODEL_NAME"),
        21059 => Some("VFX_EMITTER_INITIALROTATION"),
        21060 => Some("VFX_CRUSTNODE_NAME"),
        21061 => Some("VFX_CRUSTNODE_REALNAME"),
        21062 => Some("VFX_CRUSTNODE_CRUSTHOOKID"),
        21063 => Some("VFX_GEOMETRY_FILE_NAME"),
        21064 => Some("VFX_EMITTER_AGENT"),
        21065 => Some("VFX_USE_VARIATION_TINT"),
        21070 => Some("VFX_DUMMY_NAME"),
        21080 => Some("VFX_GEOMETRY_NAME"),
        21081 => Some("VFX_GEOMETRY_SCALE"),
        21090 => Some("VFX_TARGET_NAME"),
        21100 => Some("VFX_MODEL_NAME"),
        21101 => Some("VFX_MODEL_RESOURCETYPE"),
        21102 => Some("VFX_MODEL_ANIMATIONNAME"),
        21110 => Some("VFX_CREATURE_NAME"),
        21111 => Some("VFX_CREATURE_URI"),
        21120 => Some("VFX_RELATIVE_POSITION_X"),
        21121 => Some("VFX_RELATIVE_POSITION_Y"),
        21122 => Some("VFX_RELATIVE_POSITION_Z"),
        21123 => Some("VFX_RELATIVE_ORIENTATION_X"),
        21124 => Some("VFX_RELATIVE_ORIENTATION_Y"),
        21125 => Some("VFX_RELATIVE_ORIENTATION_Z"),
        21130 => Some("VFX_IMPACT_LENGTH"),
        21131 => Some("VFX_DURATION_LENGTH"),
        21132 => Some("VFX_CESSATION_LENGTH"),
        21133 => Some("VFX_CUSTOM_LENGTH"),
        21134 => Some("VFX_CUSTOM_NAME"),
        21140 => Some("VFX_AGEMAP_COLOR_R"),
        21141 => Some("VFX_AGEMAP_COLOR_G"),
        21142 => Some("VFX_AGEMAP_COLOR_B"),
        21143 => Some("VFX_AGEMAP_COLOR_A"),
        21144 => Some("VFX_AGEMAP_SCALE_X"),
        21145 => Some("VFX_AGEMAP_SCALE_Y"),
        21146 => Some("VFX_AGEMAP_ROTATIONAL_SPEED_MULTIPLIER"),
        21150 => Some("VFX_EVENT"),
        21151 => Some("VFX_EVENT_TIME"),
        21152 => Some("VFX_EVENT_TYPE"),
        21153 => Some("VFX_EVENT_ID"),
        21154 => Some("VFX_EVENT_TARGETSYSTEM"),
        21160 => Some("VFX_EMITTER_VOLUME_SPAWN_TYPE"),
        21161 => Some("VFX_EMITTER_VOLUME_SPAWN_SELECTED_PART_NAME"),
        21162 => Some("VFX_EMITTER_VOLUME_SPAWN_ARBITRARY_VOLUME_NAME"),
        21163 => Some("VFX_EMITTER_COLLISION_TYPE"),
        21164 => Some("VFX_EMITTER_BOUNCE_VALUE"),
        21165 => Some("VFX_EMITTER_VOLUME_SPAWN_WITHIN_VOLUME"),
        21166 => Some("VFX_EMITTER_VOLUME_SPAWN_INVERT_NORMALS"),
        21170 => Some("VFX_EMITTER_COLORMULTIPLIER_R"),
        21171 => Some("VFX_EMITTER_COLORMULTIPLIER_G"),
        21172 => Some("VFX_EMITTER_COLORMULTIPLIER_B"),
        21173 => Some("VFX_SPLAT_AGEMAP_COLOR_R"),
        21174 => Some("VFX_SPLAT_AGEMAP_COLOR_G"),
        21175 => Some("VFX_SPLAT_AGEMAP_COLOR_B"),
        21176 => Some("VFX_SPLAT_AGEMAP_COLOR_A"),
        21177 => Some("VFX_SPLAT_AGEMAP_SCALE_X"),
        21178 => Some("VFX_SPLAT_AGEMAP_SCALE_Y"),
        21180 => Some("VFX_FILE_OBJECT_VERSION"),
        21181 => Some("VFX_EMITTER_SPLAT_ALPHAMULTIPLIER"),
        21182 => Some("VFX_EMITTER_SPLAT_COLORMULTIPLIER_R"),
        21183 => Some("VFX_EMITTER_SPLAT_COLORMULTIPLIER_G"),
        21184 => Some("VFX_EMITTER_SPLAT_COLORMULTIPLIER_B"),
        21185 => Some("VFX_EMITTER_VOLUME_SPAWN_PRIMITIVE_MESH_TYPE"),
        21186 => Some("VFX_EMITTER_VOLUME_SPAWN_PRIMITIVE_BOX_MIN"),
        21187 => Some("VFX_EMITTER_VOLUME_SPAWN_PRIMITIVE_BOX_MAX"),
        21188 => Some("VFX_EMITTER_VOLUME_SPAWN_PRIMITIVE_SPHERE_R"),
        21189 => Some("VFX_EMITTER_VOLUME_SPAWN_PRIMITIVE_CYLINDER_R"),
        21190 => Some("VFX_EMITTER_VOLUME_SPAWN_PRIMITIVE_CYLINDER_H"),
        21191 => Some("VFX_EMITTER_VOLUME_SPAWN_PRIMITIVE_CYLINDER_AXIS"),
        21192 => Some("VFX_EMITTER_VOLUME_SPAWN_USE_VOLUME_NORMAL"),
        21193 => Some("VFX_EMITTER_WORLD_AXIS_ACCELERATION_X"),
        21194 => Some("VFX_EMITTER_WORLD_AXIS_ACCELERATION_Y"),
        21195 => Some("VFX_EMITTER_WORLD_AXIS_ACCELERATION_Z"),
        21196 => Some("VFX_RANGE"),
        21197 => Some("VFX_EMITTER_AXIS_ACCELERATION_SPACE"),
        21198 => Some("VFX_EMITTER_UVDISTRIBUTIONSIZE"),
        21210 => Some("VFX_EMITTER_GROUP_NAME"),
        21220 => Some("VFX_REMOTE_MATERIAL_TINT_R"),
        21221 => Some("VFX_REMOTE_MATERIAL_TINT_G"),
        21222 => Some("VFX_REMOTE_MATERIAL_TINT_B"),
        21223 => Some("VFX_REMOTE_MATERIAL_TINT_A"),
        21224 => Some("VFX_REMOTE_MATERIAL_FRESNEL_FALLOFF"),
        21225 => Some("VFX_REMOTE_MATERIAL_INVERT_FRESNEL"),
        21226 => Some("VFX_REMOTE_MATERIAL_ALPHA"),
        21227 => Some("VFX_REMOTE_MATERIAL_DECAL_NAME"),
        22000 => Some("WND_ROOT"),
        22001 => Some("WND_RESREF"),
        22002 => Some("WND_RADIUS"),
        22003 => Some("WND_STRENGTH"),
        22004 => Some("WND_DIRECTION"),
        22005 => Some("WND_RADIUS_FALLOFF"),
        22010 => Some("WND_GUST_MIN_STRENGTH"),
        22011 => Some("WND_GUST_MAX_STRENGTH"),
        22012 => Some("WND_GUST_MIN_DURATION"),
        22013 => Some("WND_GUST_MAX_DURATION"),
        22014 => Some("WND_GUST_FREQUENCY"),
        22020 => Some("WND_TREE_NUM_WIND_MATRICES"),
        22021 => Some("WND_TREE_NUM_LEAF_ANGLES"),
        22022 => Some("WND_TREE_RESPONSE"),
        22023 => Some("WND_TREE_RESPONSE_LIMIT"),
        22024 => Some("WND_TREE_MAX_BEND_ANGLE"),
        22025 => Some("WND_TREE_BRANCH_EXPONENT"),
        22026 => Some("WND_TREE_LEAF_EXPONENT"),
        22027 => Some("WND_TREE_BRANCH_OSCILLATION_X"),
        22028 => Some("WND_TREE_BRANCH_OSCILLATION_Y"),
        22029 => Some("WND_TREE_LEAF_ROCKING"),
        22030 => Some("WND_TREE_LEAF_RUSTLING"),
        22031 => Some("WND_CLOTH_RESPONSE"),
        22032 => Some("WND_CLOTH_RESPONSE_LMT"),
        22033 => Some("WND_CLOTH_STRENGTH"),
        22034 => Some("WND_CLOTH_GUST_STRENGTH_MIN"),
        22035 => Some("WND_CLOTH_GUST_STRENGTH_MAX"),
        22036 => Some("WND_CLOTH_GUST_DURATION_MIN"),
        22037 => Some("WND_CLOTH_GUST_DURATION_MAX"),
        22038 => Some("WND_CLOTH_GUST_INTERVAL_MIN"),
        22039 => Some("WND_CLOTH_GUST_INTERVAL_MAX"),
        22040 => Some("WND_CLOTH_GUST_DIR_CHANGE"),
        22041 => Some("WND_CLOTH_GUST_AXIS_RATIO"),
        22500 => Some("ATMO_DATA"),
        22519 => Some("ATMO_SUN_COLOR"),
        22520 => Some("ATMO_SUN_INTENSITY"),
        22521 => Some("ATMO_TURBIDITY"),
        22522 => Some("ATMO_EARTH_REFLECTANCE"),
        22523 => Some("ATMO_MIE_MULTIPLIER"),
        22524 => Some("ATMO_RAYLEIGH_MULTIPLIER"),
        22525 => Some("ATMO_EARTH_IN_SCATTER_POWER"),
        22526 => Some("ATMO_DISTANCE_MULTIPLIER"),
        22527 => Some("ATMO_PHASE_ECCENTRICITY"),
        22528 => Some("ATMO_ALPHA"),
        22529 => Some("ATMO_FOG_COLOR"),
        22530 => Some("ATMO_FOG_INTENSITY"),
        22531 => Some("ATMO_FOG_CAP"),
        22532 => Some("ATMO_FOG_ZENITH"),
        22533 => Some("ATMO_FOG_WATER_INTENSITY"),
        22534 => Some("ATMO_FOG_WATER_CAP"),
        22535 => Some("ATMO_FOG_TACTICAL_MULTIPLIER"),
        22600 => Some("CLOUD_DATA"),
        22620 => Some("CLOUD_DENSITY"),
        22621 => Some("CLOUD_SHARPNESS"),
        22622 => Some("CLOUD_DEPTH"),
        22623 => Some("CLOUD_RANGE_MULTIPLIER1"),
        22624 => Some("CLOUD_RANGE_MULTIPLIER2"),
        22625 => Some("CLOUD_COLOR"),
        22700 => Some("MOON_SCALE"),
        22701 => Some("MOON_ALPHA"),
        22702 => Some("MOON_CLOUDALPHA"),
        22703 => Some("MOON_ROTATION"),
        23000 => Some("MORPH_PARTS"),
        23001 => Some("MORPH_TINTFILENAMES"),
        23002 => Some("MORPH_NODES"),
        23003 => Some("MORPH_TEXTURE_NAME"),
        23004 => Some("MORPH_TEXTUREPARAM"),
        23005 => Some("MORPH_VECTOR4FPARAM"),
        23006 => Some("MORPH_FLOATPARAM"),
        23007 => Some("MORPH_FLOATPARAMVALUE"),
        23008 => Some("MORPH_NAME"),
        23009 => Some("MORPH_MAT_NODE_NAME"),
        23010 => Some("MORPH_MAT_PARAM_NAME"),
        23011 => Some("MORPH_MAT_PARAM_INDEX"),
        23012 => Some("MORPH_MAT_PARAM_VALUE"),
        23013 => Some("MORPH_MAT_PARAM_VECTOR"),
        23014 => Some("MORPH_MAT_PARAMS"),
        23015 => Some("MORPH_MAT_VEC_PARAMS"),
        23016 => Some("MORPH_MODEL_NAME"),
        23017 => Some("MORPH_MODEL_VALUE"),
        23018 => Some("MORPH_MODEL_PARAMS"),
        23019 => Some("MORPH_TEX_NODE_NAME"),
        23020 => Some("MORPH_TEX_PARAM_NAME"),
        23021 => Some("MORPH_TEX_NAME"),
        23022 => Some("MORPH_TEXTURES"),
        24000 => Some("MAP_TAG"),
        24001 => Some("MAP_TYPE"),
        24002 => Some("MAP_PINLIST"),
        24003 => Some("MAP_PIN_STATE"),
        24004 => Some("MAP_PIN_POS_X"),
        24005 => Some("MAP_PIN_POS_Y"),
        24006 => Some("MAP_PIN_NAME"),
        24007 => Some("MAP_PIN_TAG"),
        24008 => Some("MAP_PIN_AREATAG"),
        24009 => Some("MAP_PIN_TERRAINTYPE"),
        24010 => Some("MAP_PIN_TYPE"),
        24011 => Some("MAP_MAPS"),
        24012 => Some("MAP_MAP_PARENT_RESREF"),
        24013 => Some("MAP_PIN_WAYPOINT_OVERRIDE"),
        24014 => Some("MAP_TRAILLIST"),
        24015 => Some("MAP_TRAIL_PIN_1_TAG"),
        24016 => Some("MAP_TRAIL_PIN_2_TAG"),
        24017 => Some("MAP_POINTLIST"),
        24018 => Some("MAP_POINT_POS_X"),
        24019 => Some("MAP_POINT_POS_Y"),
        24020 => Some("MAP_PIN_TOOLTIP"),
        25000 => Some("DEP_FILE_LIST"),
        25001 => Some("DEP_RESREF"),
        25002 => Some("DEP_DEPENDENCY_LIST"),
        26000 => Some("SAVEPROFILE_BUILD_NUMBER"),
        26001 => Some("SAVEPROFILE_INITIAL_BUILD_NUMBER"),
        26002 => Some("SAVEPROFILE_LAST_USED_PROFILE"),
        26003 => Some("SAVEPROFILE_PROFILELIST"),
        26004 => Some("SAVEPROFILE_ACCOUNT_NAME"),
        26005 => Some("SAVEPROFILE_LOCAL_ACHIEVEMENT_DATA"),
        26006 => Some("SAVEPROFILE_ACHIEVEMENTLIST"),
        26007 => Some("SAVEPROFILE_ACHIEVEMENT_ID"),
        26008 => Some("SAVEPROFILE_ACHIEVEMENT_NEW"),
        26009 => Some("SAVEPROFILE_ACHIEVEMENT_ONLINE"),
        26010 => Some("SAVEPROFILE_ACHIEVEMENT_COUNT"),
        26011 => Some("SAVEPROFILE_ACHIEVEMENT_DATE"),
        26100 => Some("SAVEPROFILE_ADDIN_LIST"),
        26101 => Some("SAVEPROFILE_OFFER_LIST"),
        26102 => Some("SAVEPROFILE_CONTENT_NAME"),
        26103 => Some("SAVEPROFILE_CONTENT_SHOWN"),
        26104 => Some("SAVEPROFILE_CONTENT_ENABLED"),
        26105 => Some("SAVEPROFILE_CONTENT_TOKEN"),
        26106 => Some("SAVEPROFILE_CONTENT_USER"),
        26107 => Some("SAVEPROFILE_FILE_LIST"),
        26108 => Some("SAVEPROFILE_FILE_NAME"),
        26109 => Some("SAVEPROFILE_FILE_DATA"),
        26110 => Some("SAVEPROFILE_FILE_VERSION"),
        26111 => Some("SAVEPROFILE_ADDIN_TOKEN_LIST"),
        250100 => Some("CHAR_MOP"),
        250101 => Some("CHAR_APP"),
        250102 => Some("CHAR_GENDER"),
        250103 => Some("CHAR_RACE"),
        250104 => Some("CHAR_CLASS"),
        250105 => Some("CHAR_BACK"),
        250106 => Some("CHAR_ATTRIBUTES"),
        250107 => Some("CHAR_ABILITIES"),
        250108 => Some("CHAR_NAME"),
        250109 => Some("CHAR_HEAD_NAME"),
        250110 => Some("CHAR_ATTRIBUTE_ID"),
        250111 => Some("CHAR_ATTRIBUTE_POINTS"),
        250112 => Some("CHAR_PORTRAIT"),
        _ => None,
    }
}

pub fn field_id_by_name(name: &str) -> Option<u32> {
    match name {
        "AC_BLENDGROUP_ANIM_LIST" => Some(9100),
        "AC_BLENDGROUP_NAME" => Some(9102),
        "AC_BLEND_GROUP_LIST" => Some(9101),
        "AC_CAPTION" => Some(9003),
        "AC_CURVE_CONTROL_POINT_LIST" => Some(9011),
        "AC_CURVE_CONTROL_POINT_TIME" => Some(9012),
        "AC_CURVE_CONTROL_POINT_VALUE" => Some(9013),
        "AC_EDGE_END_ID" => Some(9002),
        "AC_EDGE_LIST" => Some(9007),
        "AC_EDGE_START_ID" => Some(9001),
        "AC_EVENT_ID" => Some(9017),
        "AC_EVENT_LIST" => Some(9015),
        "AC_EVENT_TIME" => Some(9016),
        "AC_FLAGS" => Some(9019),
        "AC_MODEL_NAME" => Some(9014),
        "AC_NODE_ANIMATION" => Some(9010),
        "AC_NODE_COLOUR" => Some(9009),
        "AC_NODE_IMAGE" => Some(9006),
        "AC_NODE_LIST" => Some(9008),
        "AC_NODE_LOOPING" => Some(9018),
        "AC_NODE_NAME" => Some(9000),
        "AC_NODE_SOCKET_LIST" => Some(9004),
        "AC_SOCKET_IS_OUTPUT" => Some(9005),
        "AC_TRANSITION_LIST" => Some(9024),
        "AC_TRANS_ANIM_LENGTH" => Some(9022),
        "AC_TRANS_ANIM_NAME" => Some(9020),
        "AC_TRANS_ANIM_START" => Some(9021),
        "AC_TRANS_LENGTH" => Some(9025),
        "AC_TRANS_TRACK_LIST" => Some(9023),
        "ANIMATION_ANIMLENGTH" => Some(4009),
        "ANIMATION_BLENDCURVE_ANIMFROM" => Some(4031),
        "ANIMATION_BLENDCURVE_ANIMTO" => Some(4032),
        "ANIMATION_BLENDCURVE_DATA" => Some(4033),
        "ANIMATION_BLENDCURVE_LIST" => Some(4034),
        "ANIMATION_COMBATRANGE" => Some(4010),
        "ANIMATION_ELEMENTSPERENTRY" => Some(4003),
        "ANIMATION_EVENT_ID" => Some(4017),
        "ANIMATION_EVENT_LIST" => Some(4020),
        "ANIMATION_EVENT_STRING" => Some(4019),
        "ANIMATION_EVENT_TARGET" => Some(4018),
        "ANIMATION_EVENT_TIME" => Some(4016),
        "ANIMATION_GENERALANIMNAME" => Some(4007),
        "ANIMATION_HASGOBANIM" => Some(4008),
        "ANIMATION_IGNORESCALE" => Some(4040),
        "ANIMATION_ISADDITIVE" => Some(4011),
        "ANIMATION_ISOVERRIDE" => Some(4012),
        "ANIMATION_KEY_DATA0" => Some(4036),
        "ANIMATION_KEY_DATA1" => Some(4037),
        "ANIMATION_KEY_DATA2" => Some(4038),
        "ANIMATION_KEY_DATA3" => Some(4039),
        "ANIMATION_KEY_TIME" => Some(4035),
        "ANIMATION_NAME" => Some(4006),
        "ANIMATION_NAME_HASH" => Some(4014),
        "ANIMATION_NODEDATA" => Some(4004),
        "ANIMATION_NODELIST" => Some(4005),
        "ANIMATION_NODENAME" => Some(4000),
        "ANIMATION_NODENAME_HASH" => Some(4015),
        "ANIMATION_OVERRIDEPRIORITY" => Some(4013),
        "ANIMATION_SOURCETYPE" => Some(4002),
        "ANIMATION_TARGET" => Some(4001),
        "ANIMATION_TREE" => Some(4021),
        "ANIMATION_TREE_NAME" => Some(4022),
        "ANIMATION_TREE_NODE" => Some(4023),
        "ANIMATION_TREE_NODE_FILE" => Some(4025),
        "ANIMATION_TREE_NODE_FIRST_CHILD" => Some(4028),
        "ANIMATION_TREE_NODE_FLAGS" => Some(4027),
        "ANIMATION_TREE_NODE_NAME" => Some(4024),
        "ANIMATION_TREE_NODE_NUM_CHILDREN" => Some(4029),
        "ANIMATION_TREE_NODE_PARENT" => Some(4030),
        "ANIMATION_TREE_NODE_WEIGHT" => Some(4026),
        "AREAGRID_ABSTRACTION_MEMORY" => Some(3096),
        "AREAGRID_ABSTRACTION_SADDR" => Some(3116),
        "AREAGRID_ABSTRACTION_SECTORS" => Some(3095),
        "AREAGRID_ABSTRACTION_SECTORSIZE" => Some(3094),
        "AREAGRID_ABSTRACTION_SNUMREG" => Some(3115),
        "AREAGRID_AREA" => Some(3110),
        "AREAGRID_BASEPOS" => Some(3090),
        "AREAGRID_CELLID" => Some(3098),
        "AREAGRID_CELLPADDING" => Some(3120),
        "AREAGRID_CELLSIZE" => Some(3088),
        "AREAGRID_CLEARANCE" => Some(3089),
        "AREAGRID_DATA" => Some(3092),
        "AREAGRID_GRIDID" => Some(3084),
        "AREAGRID_GRIDNAVINFO" => Some(3082),
        "AREAGRID_HEIGHT" => Some(3093),
        "AREAGRID_ID" => Some(3097),
        "AREAGRID_LIGHT_SUBSET_DATA16" => Some(3118),
        "AREAGRID_LIGHT_SUBSET_DATA8" => Some(3117),
        "AREAGRID_MODELGRID" => Some(3083),
        "AREAGRID_MODELID" => Some(3085),
        "AREAGRID_NAVINFO" => Some(3080),
        "AREAGRID_NBCOLUMNS" => Some(3086),
        "AREAGRID_NBROWS" => Some(3087),
        "AREAGRID_NORMAL" => Some(3091),
        "AREAGRID_ROOMNAME" => Some(3081),
        "AREAGRID_SOUND_DATA" => Some(3114),
        "ATMO_ALPHA" => Some(22528),
        "ATMO_DATA" => Some(22500),
        "ATMO_DISTANCE_MULTIPLIER" => Some(22526),
        "ATMO_EARTH_IN_SCATTER_POWER" => Some(22525),
        "ATMO_EARTH_REFLECTANCE" => Some(22522),
        "ATMO_FOG_CAP" => Some(22531),
        "ATMO_FOG_COLOR" => Some(22529),
        "ATMO_FOG_INTENSITY" => Some(22530),
        "ATMO_FOG_TACTICAL_MULTIPLIER" => Some(22535),
        "ATMO_FOG_WATER_CAP" => Some(22534),
        "ATMO_FOG_WATER_INTENSITY" => Some(22533),
        "ATMO_FOG_ZENITH" => Some(22532),
        "ATMO_MIE_MULTIPLIER" => Some(22523),
        "ATMO_PHASE_ECCENTRICITY" => Some(22527),
        "ATMO_RAYLEIGH_MULTIPLIER" => Some(22524),
        "ATMO_SUN_COLOR" => Some(22519),
        "ATMO_SUN_INTENSITY" => Some(22520),
        "ATMO_TURBIDITY" => Some(22521),
        "CAMPAIGN_CIF_DESCRIPTION_CS_CZ" => Some(17123),
        "CAMPAIGN_CIF_DESCRIPTION_DE_DE" => Some(17117),
        "CAMPAIGN_CIF_DESCRIPTION_EN_US" => Some(17115),
        "CAMPAIGN_CIF_DESCRIPTION_ES_ES" => Some(17121),
        "CAMPAIGN_CIF_DESCRIPTION_FR_FR" => Some(17116),
        "CAMPAIGN_CIF_DESCRIPTION_HU_HU" => Some(17122),
        "CAMPAIGN_CIF_DESCRIPTION_IT_IT" => Some(17120),
        "CAMPAIGN_CIF_DESCRIPTION_PL_PL" => Some(17118),
        "CAMPAIGN_CIF_DESCRIPTION_RU_RU" => Some(17119),
        "CAMPAIGN_CIF_DISPLAY_NAME_CS_CZ" => Some(17114),
        "CAMPAIGN_CIF_DISPLAY_NAME_DE_DE" => Some(17108),
        "CAMPAIGN_CIF_DISPLAY_NAME_EN_US" => Some(17106),
        "CAMPAIGN_CIF_DISPLAY_NAME_ES_ES" => Some(17112),
        "CAMPAIGN_CIF_DISPLAY_NAME_FR_FR" => Some(17107),
        "CAMPAIGN_CIF_DISPLAY_NAME_HU_HU" => Some(17113),
        "CAMPAIGN_CIF_DISPLAY_NAME_IT_IT" => Some(17111),
        "CAMPAIGN_CIF_DISPLAY_NAME_PL_PL" => Some(17109),
        "CAMPAIGN_CIF_DISPLAY_NAME_RU_RU" => Some(17110),
        "CAMPAIGN_CIF_ENTRY_AREA" => Some(17101),
        "CAMPAIGN_CIF_ENTRY_AREA_LIST" => Some(17100),
        "CAMPAIGN_CIF_ENTRY_CLIENT_SCRIPT" => Some(17105),
        "CAMPAIGN_CIF_ENTRY_ORIENTATION" => Some(17103),
        "CAMPAIGN_CIF_ENTRY_POSITION" => Some(17102),
        "CAMPAIGN_CIF_ENTRY_SCRIPT" => Some(17104),
        "CAMPAIGN_CIF_PACKAGES_LIST" => Some(17124),
        "CHAR_ABILITIES" => Some(250107),
        "CHAR_APP" => Some(250101),
        "CHAR_ATTRIBUTES" => Some(250106),
        "CHAR_ATTRIBUTE_ID" => Some(250110),
        "CHAR_ATTRIBUTE_POINTS" => Some(250111),
        "CHAR_BACK" => Some(250105),
        "CHAR_CLASS" => Some(250104),
        "CHAR_GENDER" => Some(250102),
        "CHAR_HEAD_NAME" => Some(250109),
        "CHAR_MOP" => Some(250100),
        "CHAR_NAME" => Some(250108),
        "CHAR_PORTRAIT" => Some(250112),
        "CHAR_RACE" => Some(250103),
        "CLOUD_COLOR" => Some(22625),
        "CLOUD_DATA" => Some(22600),
        "CLOUD_DENSITY" => Some(22620),
        "CLOUD_DEPTH" => Some(22622),
        "CLOUD_RANGE_MULTIPLIER1" => Some(22623),
        "CLOUD_RANGE_MULTIPLIER2" => Some(22624),
        "CLOUD_SHARPNESS" => Some(22621),
        "COLOR4F_LIST" => Some(20),
        "CONVERSATION_END" => Some(12003),
        "CONVERSATION_KEY_TAG" => Some(12102),
        "CONVERSATION_LINE_ACTION" => Some(12209),
        "CONVERSATION_LINE_ACTIVE" => Some(12500),
        "CONVERSATION_LINE_AMBIENT" => Some(12207),
        "CONVERSATION_LINE_ANIMATION" => Some(12213),
        "CONVERSATION_LINE_CHILDREN_LIST" => Some(12400),
        "CONVERSATION_LINE_COND" => Some(12208),
        "CONVERSATION_LINE_CUTSCENE" => Some(12211),
        "CONVERSATION_LINE_CUTSCENE_MAP" => Some(12212),
        "CONVERSATION_LINE_CUTSCENE_RESREF" => Some(12210),
        "CONVERSATION_LINE_FASTPATH" => Some(12215),
        "CONVERSATION_LINE_GAME_LANGUAGE" => Some(12204),
        "CONVERSATION_LINE_ICON" => Some(12205),
        "CONVERSATION_LINE_LIST" => Some(12002),
        "CONVERSATION_LINE_LISTENER" => Some(12203),
        "CONVERSATION_LINE_NOVOINGAME" => Some(12216),
        "CONVERSATION_LINE_REVERT_ANIM" => Some(12217),
        "CONVERSATION_LINE_SKIP" => Some(12214),
        "CONVERSATION_LINE_SLIDE_SHOW_TEXTURE" => Some(12218),
        "CONVERSATION_LINE_SPEAKER" => Some(12202),
        "CONVERSATION_LINE_TEXT" => Some(12201),
        "CONVERSATION_LINE_VISIBILITY" => Some(12206),
        "CONVERSATION_PLOT_FLAG" => Some(12301),
        "CONVERSATION_PLOT_GUID" => Some(12300),
        "CONVERSATION_PLOT_TEST" => Some(12302),
        "CONVERSATION_SCRIPT" => Some(12303),
        "CONVERSATION_SCRIPT_PARAMETER" => Some(12304),
        "CONVERSATION_STAGE_AT_CURRENT_LOCATION" => Some(12104),
        "CONVERSATION_STAGE_MAP" => Some(12101),
        "CONVERSATION_STAGE_NAME" => Some(12100),
        "CONVERSATION_STARTING_INDEX" => Some(12001),
        "CONVERSATION_STARTING_LIST" => Some(12000),
        "CONVERSATION_VALUE_TAG" => Some(12103),
        "CONVERSATION_VOBANK" => Some(12004),
        "CUTSCENE_ACTION_ACTIVE_CAMERA_ACTOR_ID" => Some(5610),
        "CUTSCENE_ACTION_ANIM_ANIMATION_NAME" => Some(5400),
        "CUTSCENE_ACTION_ANIM_APPLY_TO_FUTURE_GADS" => Some(5413),
        "CUTSCENE_ACTION_ANIM_BLENDTREE_NAME" => Some(5401),
        "CUTSCENE_ACTION_ANIM_BLEND_GAD" => Some(5410),
        "CUTSCENE_ACTION_ANIM_DEPRECATED1" => Some(5404),
        "CUTSCENE_ACTION_ANIM_EXTEND_GAD" => Some(5411),
        "CUTSCENE_ACTION_ANIM_GAD_KEYS_ORIENTATION" => Some(5409),
        "CUTSCENE_ACTION_ANIM_GAD_KEYS_POSITION" => Some(5408),
        "CUTSCENE_ACTION_ANIM_LINK_TO_MOVEMENT" => Some(5407),
        "CUTSCENE_ACTION_ANIM_LINK_TO_MOVEMENT_DISTANCES" => Some(5412),
        "CUTSCENE_ACTION_ANIM_PLAY_GAD" => Some(5405),
        "CUTSCENE_ACTION_ANIM_POSE_ANIMATION" => Some(5406),
        "CUTSCENE_ACTION_ANIM_SPEED" => Some(5402),
        "CUTSCENE_ACTION_ANIM_START_OFFSET" => Some(5403),
        "CUTSCENE_ACTION_APPLYCRUST_TARGET_ID" => Some(5640),
        "CUTSCENE_ACTION_CATEGORY" => Some(5304),
        "CUTSCENE_ACTION_CHANGEVISIBILITY_VISIBLE" => Some(5680),
        "CUTSCENE_ACTION_CURVES" => Some(5303),
        "CUTSCENE_ACTION_CURVE_BASE_VALUE" => Some(5350),
        "CUTSCENE_ACTION_CURVE_DEPRECATED" => Some(5353),
        "CUTSCENE_ACTION_CURVE_TRANSITIONS" => Some(5352),
        "CUTSCENE_ACTION_CURVE_TRANSITION_CONTROL_1" => Some(5381),
        "CUTSCENE_ACTION_CURVE_TRANSITION_CONTROL_2" => Some(5382),
        "CUTSCENE_ACTION_CURVE_TRANSITION_TYPE" => Some(5380),
        "CUTSCENE_ACTION_CURVE_VERTEX_TIME" => Some(5370),
        "CUTSCENE_ACTION_CURVE_VERTEX_VALUE" => Some(5371),
        "CUTSCENE_ACTION_CURVE_VERTICES" => Some(5351),
        "CUTSCENE_ACTION_DRAW_WEAPON_MAIN" => Some(5730),
        "CUTSCENE_ACTION_DRAW_WEAPON_OFF" => Some(5731),
        "CUTSCENE_ACTION_FRAME_BUFFER_EFFECT_EFFECT_NAME" => Some(5521),
        "CUTSCENE_ACTION_FRAME_BUFFER_EFFECT_FILE_NAME" => Some(5520),
        "CUTSCENE_ACTION_FRAME_BUFFER_EFFECT_PARAM_CURVE_INDEX" => Some(5525),
        "CUTSCENE_ACTION_FRAME_BUFFER_EFFECT_PARAM_LIST" => Some(5522),
        "CUTSCENE_ACTION_FRAME_BUFFER_EFFECT_PARAM_NAME" => Some(5523),
        "CUTSCENE_ACTION_FRAME_BUFFER_EFFECT_PARAM_VALUE" => Some(5524),
        "CUTSCENE_ACTION_FRAME_BUFFER_EFFECT_PARAM_VECTOR_INDEX" => Some(5526),
        "CUTSCENE_ACTION_HEADTRACKING_DEPRECATED1" => Some(5624),
        "CUTSCENE_ACTION_HEADTRACKING_DEPRECATED2" => Some(5625),
        "CUTSCENE_ACTION_HEADTRACKING_DEPRECATED3" => Some(5626),
        "CUTSCENE_ACTION_HEADTRACKING_DEPRECATED4" => Some(5627),
        "CUTSCENE_ACTION_HEADTRACKING_REALIGN_CONT" => Some(5629),
        "CUTSCENE_ACTION_HEADTRACKING_REALIGN_START" => Some(5628),
        "CUTSCENE_ACTION_HEADTRACKING_SPEED" => Some(5621),
        "CUTSCENE_ACTION_HEADTRACKING_TARGET_ID" => Some(5620),
        "CUTSCENE_ACTION_LINK_ACTOR_DEPRECATED1" => Some(5632),
        "CUTSCENE_ACTION_LINK_ACTOR_DEPRECATED2" => Some(5633),
        "CUTSCENE_ACTION_LINK_ACTOR_DEPRECATED3" => Some(5634),
        "CUTSCENE_ACTION_LINK_ACTOR_DEPRECATED4" => Some(5635),
        "CUTSCENE_ACTION_LINK_ACTOR_IS_TARGET_CRUST" => Some(5636),
        "CUTSCENE_ACTION_LINK_ACTOR_NODE_ID" => Some(5631),
        "CUTSCENE_ACTION_LINK_ACTOR_TARGET_ID" => Some(5630),
        "CUTSCENE_ACTION_LINK_ACTOR_USE_OFFSET" => Some(5637),
        "CUTSCENE_ACTION_PLAYMOVIE" => Some(5740),
        "CUTSCENE_ACTION_POSE_ANIMATION_ANIMATION" => Some(5651),
        "CUTSCENE_ACTION_POSE_ANIMATION_LOOPING" => Some(5652),
        "CUTSCENE_ACTION_POSE_ANIMATION_OUTRO" => Some(5653),
        "CUTSCENE_ACTION_POSE_ANIMATION_OUTRO_SPEED" => Some(5654),
        "CUTSCENE_ACTION_POSE_ANIMATION_POSE" => Some(5650),
        "CUTSCENE_ACTION_SETGORE" => Some(5750),
        "CUTSCENE_ACTION_SET_LOD_DEPRECATED" => Some(5720),
        "CUTSCENE_ACTION_SHAKE_DEPRECATED1" => Some(5601),
        "CUTSCENE_ACTION_SHAKE_DEPRECATED2" => Some(5602),
        "CUTSCENE_ACTION_SHAKE_NOISE_CORRELATED" => Some(5606),
        "CUTSCENE_ACTION_SHAKE_NOISE_FREQUENCY" => Some(5604),
        "CUTSCENE_ACTION_SHAKE_NOISE_RAMP_IN" => Some(5608),
        "CUTSCENE_ACTION_SHAKE_NOISE_RAMP_OUT" => Some(5609),
        "CUTSCENE_ACTION_SHAKE_NOISE_ROUGHNESS" => Some(5607),
        "CUTSCENE_ACTION_SHAKE_NOISE_SEED" => Some(5603),
        "CUTSCENE_ACTION_SHAKE_NOISE_TYPE" => Some(5605),
        "CUTSCENE_ACTION_SHAKE_TYPE" => Some(5600),
        "CUTSCENE_ACTION_SOUND_NAME" => Some(5670),
        "CUTSCENE_ACTION_SOUND_SPLINE_PARAM_NO1" => Some(5671),
        "CUTSCENE_ACTION_SOUND_SPLINE_PARAM_NO2" => Some(5672),
        "CUTSCENE_ACTION_SOUND_SPLINE_PARAM_NO3" => Some(5673),
        "CUTSCENE_ACTION_SOUND_SPLINE_PARAM_NO4" => Some(5674),
        "CUTSCENE_ACTION_SOUND_SPLINE_PARAM_NO5" => Some(5675),
        "CUTSCENE_ACTION_SPEAK_LINE_FAHEADMOVEMENT" => Some(5564),
        "CUTSCENE_ACTION_SPEAK_LINE_LIPSYNCH_SET" => Some(5562),
        "CUTSCENE_ACTION_SPEAK_LINE_NOVOINGAME" => Some(5565),
        "CUTSCENE_ACTION_SPEAK_LINE_VOBANK" => Some(5563),
        "CUTSCENE_ACTION_STAGE_CAMERA_DEFAULT_CAMERA" => Some(5570),
        "CUTSCENE_ACTION_STAGE_CAMERA_HENCHMAN_CAMERA" => Some(5571),
        "CUTSCENE_ACTION_STAGE_PLACE_LOOK_AT" => Some(5580),
        "CUTSCENE_ACTION_START_TIME" => Some(5301),
        "CUTSCENE_ACTION_STOP_TIME" => Some(5302),
        "CUTSCENE_ACTION_TOGGLE_CLOTH_PHYSICS" => Some(5700),
        "CUTSCENE_ACTION_TOGGLE_HAIR_PHYSICS" => Some(5701),
        "CUTSCENE_ACTION_TYPE" => Some(5300),
        "CUTSCENE_ACTORS" => Some(5200),
        "CUTSCENE_ACTOR_ACTION_QUEUE" => Some(5206),
        "CUTSCENE_ACTOR_AMBIENT_ANIM" => Some(5225),
        "CUTSCENE_ACTOR_CAMERA_TARGET" => Some(5209),
        "CUTSCENE_ACTOR_CREATURE_RESREF" => Some(5208),
        "CUTSCENE_ACTOR_DEPRECATED_1" => Some(5203),
        "CUTSCENE_ACTOR_DEPRECATED_2" => Some(5204),
        "CUTSCENE_ACTOR_DEPRECATED_3" => Some(5205),
        "CUTSCENE_ACTOR_DEPRECATED_4" => Some(5207),
        "CUTSCENE_ACTOR_FINAL_ORI" => Some(5222),
        "CUTSCENE_ACTOR_FINAL_POS" => Some(5221),
        "CUTSCENE_ACTOR_ID" => Some(5201),
        "CUTSCENE_ACTOR_INVENTORY" => Some(5217),
        "CUTSCENE_ACTOR_LOD" => Some(5224),
        "CUTSCENE_ACTOR_MAPPING_REQUIRED" => Some(5220),
        "CUTSCENE_ACTOR_MAPPING_TAG" => Some(5216),
        "CUTSCENE_ACTOR_MASTER" => Some(5223),
        "CUTSCENE_ACTOR_MODEL_RESREF" => Some(5202),
        "CUTSCENE_ACTOR_MODEL_SCALE" => Some(5226),
        "CUTSCENE_ACTOR_ORIGIN_ORI" => Some(5215),
        "CUTSCENE_ACTOR_ORIGIN_POS" => Some(5214),
        "CUTSCENE_ACTOR_POSE" => Some(5211),
        "CUTSCENE_ACTOR_POSE_HUMANOID" => Some(5213),
        "CUTSCENE_ACTOR_POSE_SPEED" => Some(5212),
        "CUTSCENE_ACTOR_PREVIOUS_POSE" => Some(5219),
        "CUTSCENE_ACTOR_TRANSITION_DELAY" => Some(5218),
        "CUTSCENE_ACTOR_USE_POSE" => Some(5210),
        "CUTSCENE_ANIMATIC" => Some(5008),
        "CUTSCENE_ANIM_SOUND_EVENTS" => Some(5012),
        "CUTSCENE_AREA_REQUIRED" => Some(5019),
        "CUTSCENE_BLENDTREE" => Some(5007),
        "CUTSCENE_ENABLE_LEVEL_FBES" => Some(5013),
        "CUTSCENE_END_SCRIPT" => Some(5001),
        "CUTSCENE_FOV" => Some(5006),
        "CUTSCENE_FPS" => Some(5016),
        "CUTSCENE_HENCHMAN_ACTIONS" => Some(5051),
        "CUTSCENE_HENCHMAN_TAG" => Some(5050),
        "CUTSCENE_LAYOUT" => Some(5002),
        "CUTSCENE_LIGHT_OCCLUSION" => Some(5021),
        "CUTSCENE_LOD_CURVES" => Some(5011),
        "CUTSCENE_LOD_ORIGIN_ORI" => Some(5015),
        "CUTSCENE_LOD_ORIGIN_POS" => Some(5014),
        "CUTSCENE_ORIENTATION" => Some(5004),
        "CUTSCENE_PLAY_UNTIL_VO_COMPLETES" => Some(5018),
        "CUTSCENE_POSITION" => Some(5003),
        "CUTSCENE_RESOURCES" => Some(5100),
        "CUTSCENE_RESOURCE_RESREF" => Some(5101),
        "CUTSCENE_RESOURCE_TYPE" => Some(5102),
        "CUTSCENE_RUN_TIME" => Some(5000),
        "CUTSCENE_SHADOW_RADIUS" => Some(5020),
        "CUTSCENE_SHOWAREADYNAMICS" => Some(5009),
        "CUTSCENE_STAGED" => Some(5010),
        "CUTSCENE_STAGE_RESREF" => Some(5017),
        "CUTSCENE_TRANSITION_TIME" => Some(5005),
        "DEP_DEPENDENCY_LIST" => Some(25002),
        "DEP_FILE_LIST" => Some(25000),
        "DEP_RESREF" => Some(25001),
        "DYNAMICSHADOW_VECTOR_GAME" => Some(3107),
        "ECSTRING_LIST" => Some(19),
        "ENV_AREA" => Some(3010),
        "ENV_AREA_CENTER" => Some(3024),
        "ENV_AREA_CHUNK_COLCOUNT" => Some(3124),
        "ENV_AREA_CHUNK_HEIGHT" => Some(3126),
        "ENV_AREA_CHUNK_ISCHUNK" => Some(3122),
        "ENV_AREA_CHUNK_ROWCOUNT" => Some(3123),
        "ENV_AREA_CHUNK_WIDTH" => Some(3125),
        "ENV_AREA_CUTOFF_HEIGHT" => Some(3129),
        "ENV_AREA_CUTOFF_SYSTEM_ENABLED" => Some(3134),
        "ENV_AREA_ENVIRONMENTSETTINGS" => Some(3014),
        "ENV_AREA_FILE" => Some(3013),
        "ENV_AREA_FORCE_CHARACTER_LIGHTING" => Some(3142),
        "ENV_AREA_FRAME_BUFFER_EFFECT" => Some(3023),
        "ENV_AREA_FRAME_BUFFER_EFFECT_LIST" => Some(3026),
        "ENV_AREA_GLOBALWIND_RESREF" => Some(3027),
        "ENV_AREA_ID" => Some(3011),
        "ENV_AREA_LAYOUT_NAME" => Some(3127),
        "ENV_AREA_LOCALWIND_LIST" => Some(3028),
        "ENV_AREA_NAME" => Some(3012),
        "ENV_AREA_NAVIGATION_INFO_FILE" => Some(3015),
        "ENV_AREA_PATHFINDING_COSTS" => Some(3029),
        "ENV_AREA_PATHFINDING_EXPORT" => Some(3020),
        "ENV_AREA_PATHFINDING_VISINFO" => Some(3021),
        "ENV_AREA_PATHFINDING_VISINFO_COUNT" => Some(3022),
        "ENV_AREA_POSITION" => Some(3018),
        "ENV_AREA_ROOM_LIST" => Some(3016),
        "ENV_AREA_ROOM_LIST_ELEMENT" => Some(3017),
        "ENV_AREA_ROTATION" => Some(3019),
        "ENV_AREA_SKYDOME_MODEL" => Some(3025),
        "ENV_AREA_STARTPOINT_NAME" => Some(3128),
        "ENV_AREA_SUNLIGHT_CAN_BE_OCCLUDED_CHAR" => Some(3148),
        "ENV_AREA_SUNLIGHT_COLOR" => Some(3152),
        "ENV_AREA_SUNLIGHT_COLORMULT" => Some(3153),
        "ENV_AREA_SUNLIGHT_COLOR_CHAR" => Some(3149),
        "ENV_AREA_SUNLIGHT_DIRECTION" => Some(3150),
        "ENV_AREA_SUNLIGHT_ENABLED" => Some(3151),
        "ENV_AREA_TREECONTROLLER_ID" => Some(3357),
        "ENV_AREA_TREECONTROLLER_LIST" => Some(3355),
        "ENV_CAMERA" => Some(3200),
        "ENV_CAMERA_PIVOTDISTANCE" => Some(3201),
        "ENV_CREATURE" => Some(3172),
        "ENV_FOG_COLOR" => Some(3165),
        "ENV_FOG_ENABLED" => Some(3168),
        "ENV_FOG_MAX_DISTANCE" => Some(3166),
        "ENV_FOG_MAX_INTENSITY" => Some(3167),
        "ENV_FOG_MIN_DISTANCE" => Some(3169),
        "ENV_GROUP" => Some(3300),
        "ENV_GROUP_NAME" => Some(3301),
        "ENV_LIGHT" => Some(3067),
        "ENV_LIGHT_ID" => Some(3068),
        "ENV_LIGHT_NAME" => Some(3069),
        "ENV_LIGHT_NUM_SAMPLES" => Some(3105),
        "ENV_LIGHT_POSITION" => Some(3070),
        "ENV_LIGHT_PROBE" => Some(3103),
        "ENV_LIGHT_PROBE_ENVMAP" => Some(3104),
        "ENV_LIGHT_PROBE_ID" => Some(3108),
        "ENV_LIGHT_ROTATION" => Some(3071),
        "ENV_LIGHT_SIZE" => Some(3106),
        "ENV_LIST_AREA" => Some(3202),
        "ENV_LIST_LIGHT" => Some(3205),
        "ENV_LIST_MODEL" => Some(3204),
        "ENV_LIST_ROOM" => Some(3203),
        "ENV_MINIMAP_LOWER_LEFT_POINT" => Some(3138),
        "ENV_MINIMAP_TEXTURE_MAP_COORDS" => Some(3137),
        "ENV_MINIMAP_UPPER_RIGHT_POINT" => Some(3139),
        "ENV_MODEL" => Some(3060),
        "ENV_MODEL_BLEND_TREE_NAME" => Some(3505),
        "ENV_MODEL_BOUNDS_CENTER" => Some(3321),
        "ENV_MODEL_BOUNDS_RADIUS" => Some(3322),
        "ENV_MODEL_CUT_AWAY_OVERRIDE" => Some(3109),
        "ENV_MODEL_DEFAULT_ANIMATION" => Some(3504),
        "ENV_MODEL_EXPORT_FLAG" => Some(3503),
        "ENV_MODEL_FILE" => Some(3063),
        "ENV_MODEL_ID" => Some(3061),
        "ENV_MODEL_INSTANCEID" => Some(3320),
        "ENV_MODEL_LIGHTMAPONLY" => Some(3501),
        "ENV_MODEL_LIGHTMAP_ATLAS" => Some(3323),
        "ENV_MODEL_LIGHTMAP_ATLAS_LIST" => Some(3326),
        "ENV_MODEL_LIGHTMAP_FLAG" => Some(3502),
        "ENV_MODEL_LIGHTMAP_OFFSET_SCALE" => Some(3324),
        "ENV_MODEL_LIGHTMAP_PART_ID" => Some(3235),
        "ENV_MODEL_NAME" => Some(3062),
        "ENV_MODEL_NAME_CHANGED" => Some(3170),
        "ENV_MODEL_PARTGROUP" => Some(3500),
        "ENV_MODEL_PATHFINDING_NORMAL" => Some(3066),
        "ENV_MODEL_PATHFINDING_OVERLAPPED" => Some(3056),
        "ENV_MODEL_POSITION" => Some(3064),
        "ENV_MODEL_ROTATION" => Some(3065),
        "ENV_MODEL_SCALE" => Some(3059),
        "ENV_MODEL_SHOW_HIGH_LOD" => Some(3057),
        "ENV_MODEL_SNAP_TO_TERRAIN" => Some(3058),
        "ENV_MODEL_USER_PARAM_LIST" => Some(3506),
        "ENV_MODEL_USER_PARAM_NAME" => Some(3507),
        "ENV_MODEL_USER_PARAM_VALUE" => Some(3508),
        "ENV_OBJECT_LOCKSELECTION" => Some(3311),
        "ENV_OBJECT_VISIBLE" => Some(3310),
        "ENV_PFCONTAINER_DATAVERSION" => Some(3212),
        "ENV_PFCONTAINER_EXPORTDATA" => Some(3211),
        "ENV_PFCONTAINER_LAYOUTNAME" => Some(3210),
        "ENV_PFCONTAINER_VISINFO" => Some(3213),
        "ENV_ROOM" => Some(3030),
        "ENV_ROOM_CONNECTIVITY_LIST" => Some(3099),
        "ENV_ROOM_DYNSHADOW_DIRECTION" => Some(3054),
        "ENV_ROOM_DYNSHADOW_ENABLED" => Some(3055),
        "ENV_ROOM_ENVIRONMENTSETTINGS" => Some(3034),
        "ENV_ROOM_FILE" => Some(3033),
        "ENV_ROOM_ID" => Some(3031),
        "ENV_ROOM_LIGHT_LIST" => Some(3051),
        "ENV_ROOM_LIGHT_LIST_ELEMENT" => Some(3053),
        "ENV_ROOM_LIGHT_VIS_LIST" => Some(3164),
        "ENV_ROOM_LOWER_LEFT_POINT" => Some(3140),
        "ENV_ROOM_MODEL_LIST" => Some(3050),
        "ENV_ROOM_MODEL_LIST_ELEMENT" => Some(3052),
        "ENV_ROOM_NAME" => Some(3032),
        "ENV_ROOM_PATHCONNECTION" => Some(3045),
        "ENV_ROOM_PATHCONNECTION_ID" => Some(3046),
        "ENV_ROOM_PATHCONNECTION_LIST" => Some(3044),
        "ENV_ROOM_PATHFINDING_CHARACTERHEIGHT" => Some(3038),
        "ENV_ROOM_PATHFINDING_CLEARANCE" => Some(3039),
        "ENV_ROOM_PATHFINDING_EXPORT" => Some(3040),
        "ENV_ROOM_PATHFINDING_GRIDSEPARATION" => Some(3037),
        "ENV_ROOM_PATHFINDING_VISINFO" => Some(3041),
        "ENV_ROOM_PATHFINDING_VISINFO_COUNT" => Some(3042),
        "ENV_ROOM_PATH_GRID_FILE" => Some(3043),
        "ENV_ROOM_POSITION" => Some(3035),
        "ENV_ROOM_ROTATION" => Some(3036),
        "ENV_ROOM_TREENODE_LIST" => Some(3354),
        "ENV_ROOM_UPPER_RIGHT_POINT" => Some(3141),
        "ENV_ROOM_VISIBILITY" => Some(3048),
        "ENV_ROOM_VISIBILITY_ID" => Some(3049),
        "ENV_ROOM_VISIBILITY_LIST" => Some(3047),
        "ENV_SCATTEROBJECT_FILE" => Some(3363),
        "ENV_SCATTEROBJECT_ID" => Some(3367),
        "ENV_SCATTEROBJECT_LIST" => Some(3366),
        "ENV_SCATTEROBJ_IGNORE_MAX_DENSITY" => Some(3368),
        "ENV_SCATTEROBJ_MAX_DENSITY" => Some(3369),
        "ENV_SCATTEROBJ_MAX_SCALE" => Some(3371),
        "ENV_SCATTEROBJ_MIN_SCALE" => Some(3370),
        "ENV_SCATTEROBJ_MSI_DATA" => Some(3374),
        "ENV_SCATTEROBJ_ORIENT" => Some(3372),
        "ENV_SCATTEROBJ_PROTOTYPE" => Some(3373),
        "ENV_SCATTEROBJ_SOUND_TYPE" => Some(3376),
        "ENV_SCATTER_INSTANCE" => Some(3364),
        "ENV_SCATTER_INSTANCE_LIST" => Some(3365),
        "ENV_SCATTER_OBJECTS" => Some(3362),
        "ENV_SP" => Some(3304),
        "ENV_SP_FILE" => Some(3305),
        "ENV_SP_GROUP" => Some(3302),
        "ENV_SP_GROUP_NAME" => Some(3303),
        "ENV_STANDALONE" => Some(3202),
        "ENV_STAT_PHYS" => Some(3744),
        "ENV_STAT_PHYS_DATA" => Some(3745),
        "ENV_TREE" => Some(3350),
        "ENV_TREENODE_ID" => Some(3351),
        "ENV_TREE_COLOR_LEVEL_INTENSITY" => Some(3378),
        "ENV_TREE_COLOR_LEVEL_TINT" => Some(3377),
        "ENV_TREE_COLOR_TINT" => Some(3375),
        "ENV_TREE_DRAW_DISTANCE" => Some(3379),
        "ENV_TREE_FILE" => Some(3353),
        "ENV_TREE_NAME" => Some(3352),
        "ENV_TREE_PAINTED_LIST" => Some(3358),
        "ENV_TREE_PAINTED_POSITION" => Some(3359),
        "ENV_TREE_PAINTED_ROTATION" => Some(3360),
        "ENV_TREE_PAINTED_SCALE" => Some(3361),
        "ENV_TREE_SCALE" => Some(3356),
        "ENV_VEGETATION" => Some(3171),
        "ENV_WORLD" => Some(3000),
        "ENV_WORLD_AREA_LIST" => Some(3002),
        "ENV_WORLD_NAME" => Some(3001),
        "FLOAT32_LIST" => Some(14),
        "FLOAT64_LIST" => Some(15),
        "G2DA_COLUMN_1" => Some(10005),
        "G2DA_COLUMN_10" => Some(10014),
        "G2DA_COLUMN_100" => Some(10104),
        "G2DA_COLUMN_101" => Some(10105),
        "G2DA_COLUMN_102" => Some(10106),
        "G2DA_COLUMN_103" => Some(10107),
        "G2DA_COLUMN_104" => Some(10108),
        "G2DA_COLUMN_105" => Some(10109),
        "G2DA_COLUMN_106" => Some(10110),
        "G2DA_COLUMN_107" => Some(10111),
        "G2DA_COLUMN_108" => Some(10112),
        "G2DA_COLUMN_109" => Some(10113),
        "G2DA_COLUMN_11" => Some(10015),
        "G2DA_COLUMN_110" => Some(10114),
        "G2DA_COLUMN_111" => Some(10115),
        "G2DA_COLUMN_112" => Some(10116),
        "G2DA_COLUMN_113" => Some(10117),
        "G2DA_COLUMN_114" => Some(10118),
        "G2DA_COLUMN_115" => Some(10119),
        "G2DA_COLUMN_116" => Some(10120),
        "G2DA_COLUMN_117" => Some(10121),
        "G2DA_COLUMN_118" => Some(10122),
        "G2DA_COLUMN_119" => Some(10123),
        "G2DA_COLUMN_12" => Some(10016),
        "G2DA_COLUMN_120" => Some(10124),
        "G2DA_COLUMN_121" => Some(10125),
        "G2DA_COLUMN_122" => Some(10126),
        "G2DA_COLUMN_123" => Some(10127),
        "G2DA_COLUMN_124" => Some(10128),
        "G2DA_COLUMN_125" => Some(10129),
        "G2DA_COLUMN_126" => Some(10130),
        "G2DA_COLUMN_127" => Some(10131),
        "G2DA_COLUMN_128" => Some(10132),
        "G2DA_COLUMN_129" => Some(10133),
        "G2DA_COLUMN_13" => Some(10017),
        "G2DA_COLUMN_130" => Some(10134),
        "G2DA_COLUMN_131" => Some(10135),
        "G2DA_COLUMN_132" => Some(10136),
        "G2DA_COLUMN_133" => Some(10137),
        "G2DA_COLUMN_134" => Some(10138),
        "G2DA_COLUMN_135" => Some(10139),
        "G2DA_COLUMN_136" => Some(10140),
        "G2DA_COLUMN_137" => Some(10141),
        "G2DA_COLUMN_138" => Some(10142),
        "G2DA_COLUMN_139" => Some(10143),
        "G2DA_COLUMN_14" => Some(10018),
        "G2DA_COLUMN_140" => Some(10144),
        "G2DA_COLUMN_141" => Some(10145),
        "G2DA_COLUMN_142" => Some(10146),
        "G2DA_COLUMN_143" => Some(10147),
        "G2DA_COLUMN_144" => Some(10148),
        "G2DA_COLUMN_145" => Some(10149),
        "G2DA_COLUMN_146" => Some(10150),
        "G2DA_COLUMN_147" => Some(10151),
        "G2DA_COLUMN_148" => Some(10152),
        "G2DA_COLUMN_149" => Some(10153),
        "G2DA_COLUMN_15" => Some(10019),
        "G2DA_COLUMN_150" => Some(10154),
        "G2DA_COLUMN_151" => Some(10155),
        "G2DA_COLUMN_152" => Some(10156),
        "G2DA_COLUMN_153" => Some(10157),
        "G2DA_COLUMN_154" => Some(10158),
        "G2DA_COLUMN_155" => Some(10159),
        "G2DA_COLUMN_156" => Some(10160),
        "G2DA_COLUMN_157" => Some(10161),
        "G2DA_COLUMN_158" => Some(10162),
        "G2DA_COLUMN_159" => Some(10163),
        "G2DA_COLUMN_16" => Some(10020),
        "G2DA_COLUMN_160" => Some(10164),
        "G2DA_COLUMN_161" => Some(10165),
        "G2DA_COLUMN_162" => Some(10166),
        "G2DA_COLUMN_163" => Some(10167),
        "G2DA_COLUMN_164" => Some(10168),
        "G2DA_COLUMN_165" => Some(10169),
        "G2DA_COLUMN_166" => Some(10170),
        "G2DA_COLUMN_167" => Some(10171),
        "G2DA_COLUMN_168" => Some(10172),
        "G2DA_COLUMN_169" => Some(10173),
        "G2DA_COLUMN_17" => Some(10021),
        "G2DA_COLUMN_170" => Some(10174),
        "G2DA_COLUMN_171" => Some(10175),
        "G2DA_COLUMN_172" => Some(10176),
        "G2DA_COLUMN_173" => Some(10177),
        "G2DA_COLUMN_174" => Some(10178),
        "G2DA_COLUMN_175" => Some(10179),
        "G2DA_COLUMN_176" => Some(10180),
        "G2DA_COLUMN_177" => Some(10181),
        "G2DA_COLUMN_178" => Some(10182),
        "G2DA_COLUMN_179" => Some(10183),
        "G2DA_COLUMN_18" => Some(10022),
        "G2DA_COLUMN_180" => Some(10184),
        "G2DA_COLUMN_181" => Some(10185),
        "G2DA_COLUMN_182" => Some(10186),
        "G2DA_COLUMN_183" => Some(10187),
        "G2DA_COLUMN_184" => Some(10188),
        "G2DA_COLUMN_185" => Some(10189),
        "G2DA_COLUMN_186" => Some(10190),
        "G2DA_COLUMN_187" => Some(10191),
        "G2DA_COLUMN_188" => Some(10192),
        "G2DA_COLUMN_189" => Some(10193),
        "G2DA_COLUMN_19" => Some(10023),
        "G2DA_COLUMN_190" => Some(10194),
        "G2DA_COLUMN_191" => Some(10195),
        "G2DA_COLUMN_192" => Some(10196),
        "G2DA_COLUMN_193" => Some(10197),
        "G2DA_COLUMN_194" => Some(10198),
        "G2DA_COLUMN_195" => Some(10199),
        "G2DA_COLUMN_196" => Some(10200),
        "G2DA_COLUMN_197" => Some(10201),
        "G2DA_COLUMN_198" => Some(10202),
        "G2DA_COLUMN_199" => Some(10203),
        "G2DA_COLUMN_2" => Some(10006),
        "G2DA_COLUMN_20" => Some(10024),
        "G2DA_COLUMN_200" => Some(10204),
        "G2DA_COLUMN_201" => Some(10205),
        "G2DA_COLUMN_202" => Some(10206),
        "G2DA_COLUMN_203" => Some(10207),
        "G2DA_COLUMN_204" => Some(10208),
        "G2DA_COLUMN_205" => Some(10209),
        "G2DA_COLUMN_206" => Some(10210),
        "G2DA_COLUMN_207" => Some(10211),
        "G2DA_COLUMN_208" => Some(10212),
        "G2DA_COLUMN_209" => Some(10213),
        "G2DA_COLUMN_21" => Some(10025),
        "G2DA_COLUMN_210" => Some(10214),
        "G2DA_COLUMN_211" => Some(10215),
        "G2DA_COLUMN_212" => Some(10216),
        "G2DA_COLUMN_213" => Some(10217),
        "G2DA_COLUMN_214" => Some(10218),
        "G2DA_COLUMN_215" => Some(10219),
        "G2DA_COLUMN_216" => Some(10220),
        "G2DA_COLUMN_217" => Some(10221),
        "G2DA_COLUMN_218" => Some(10222),
        "G2DA_COLUMN_219" => Some(10223),
        "G2DA_COLUMN_22" => Some(10026),
        "G2DA_COLUMN_220" => Some(10224),
        "G2DA_COLUMN_221" => Some(10225),
        "G2DA_COLUMN_222" => Some(10226),
        "G2DA_COLUMN_223" => Some(10227),
        "G2DA_COLUMN_224" => Some(10228),
        "G2DA_COLUMN_225" => Some(10229),
        "G2DA_COLUMN_226" => Some(10230),
        "G2DA_COLUMN_227" => Some(10231),
        "G2DA_COLUMN_228" => Some(10232),
        "G2DA_COLUMN_229" => Some(10233),
        "G2DA_COLUMN_23" => Some(10027),
        "G2DA_COLUMN_230" => Some(10234),
        "G2DA_COLUMN_231" => Some(10235),
        "G2DA_COLUMN_232" => Some(10236),
        "G2DA_COLUMN_233" => Some(10237),
        "G2DA_COLUMN_234" => Some(10238),
        "G2DA_COLUMN_235" => Some(10239),
        "G2DA_COLUMN_236" => Some(10240),
        "G2DA_COLUMN_237" => Some(10241),
        "G2DA_COLUMN_238" => Some(10242),
        "G2DA_COLUMN_239" => Some(10243),
        "G2DA_COLUMN_24" => Some(10028),
        "G2DA_COLUMN_240" => Some(10244),
        "G2DA_COLUMN_241" => Some(10245),
        "G2DA_COLUMN_242" => Some(10246),
        "G2DA_COLUMN_243" => Some(10247),
        "G2DA_COLUMN_244" => Some(10248),
        "G2DA_COLUMN_245" => Some(10249),
        "G2DA_COLUMN_246" => Some(10250),
        "G2DA_COLUMN_247" => Some(10251),
        "G2DA_COLUMN_248" => Some(10252),
        "G2DA_COLUMN_249" => Some(10253),
        "G2DA_COLUMN_25" => Some(10029),
        "G2DA_COLUMN_250" => Some(10254),
        "G2DA_COLUMN_251" => Some(10255),
        "G2DA_COLUMN_252" => Some(10256),
        "G2DA_COLUMN_253" => Some(10257),
        "G2DA_COLUMN_254" => Some(10258),
        "G2DA_COLUMN_255" => Some(10259),
        "G2DA_COLUMN_256" => Some(10260),
        "G2DA_COLUMN_257" => Some(10261),
        "G2DA_COLUMN_258" => Some(10262),
        "G2DA_COLUMN_259" => Some(10263),
        "G2DA_COLUMN_26" => Some(10030),
        "G2DA_COLUMN_260" => Some(10264),
        "G2DA_COLUMN_261" => Some(10265),
        "G2DA_COLUMN_262" => Some(10266),
        "G2DA_COLUMN_263" => Some(10267),
        "G2DA_COLUMN_264" => Some(10268),
        "G2DA_COLUMN_265" => Some(10269),
        "G2DA_COLUMN_266" => Some(10270),
        "G2DA_COLUMN_267" => Some(10271),
        "G2DA_COLUMN_268" => Some(10272),
        "G2DA_COLUMN_269" => Some(10273),
        "G2DA_COLUMN_27" => Some(10031),
        "G2DA_COLUMN_270" => Some(10274),
        "G2DA_COLUMN_271" => Some(10275),
        "G2DA_COLUMN_272" => Some(10276),
        "G2DA_COLUMN_273" => Some(10277),
        "G2DA_COLUMN_274" => Some(10278),
        "G2DA_COLUMN_275" => Some(10279),
        "G2DA_COLUMN_276" => Some(10280),
        "G2DA_COLUMN_277" => Some(10281),
        "G2DA_COLUMN_278" => Some(10282),
        "G2DA_COLUMN_279" => Some(10283),
        "G2DA_COLUMN_28" => Some(10032),
        "G2DA_COLUMN_280" => Some(10284),
        "G2DA_COLUMN_281" => Some(10285),
        "G2DA_COLUMN_282" => Some(10286),
        "G2DA_COLUMN_283" => Some(10287),
        "G2DA_COLUMN_284" => Some(10288),
        "G2DA_COLUMN_285" => Some(10289),
        "G2DA_COLUMN_286" => Some(10290),
        "G2DA_COLUMN_287" => Some(10291),
        "G2DA_COLUMN_288" => Some(10292),
        "G2DA_COLUMN_289" => Some(10293),
        "G2DA_COLUMN_29" => Some(10033),
        "G2DA_COLUMN_290" => Some(10294),
        "G2DA_COLUMN_291" => Some(10295),
        "G2DA_COLUMN_292" => Some(10296),
        "G2DA_COLUMN_293" => Some(10297),
        "G2DA_COLUMN_294" => Some(10298),
        "G2DA_COLUMN_295" => Some(10299),
        "G2DA_COLUMN_296" => Some(10300),
        "G2DA_COLUMN_297" => Some(10301),
        "G2DA_COLUMN_298" => Some(10302),
        "G2DA_COLUMN_299" => Some(10303),
        "G2DA_COLUMN_3" => Some(10007),
        "G2DA_COLUMN_30" => Some(10034),
        "G2DA_COLUMN_31" => Some(10035),
        "G2DA_COLUMN_32" => Some(10036),
        "G2DA_COLUMN_33" => Some(10037),
        "G2DA_COLUMN_34" => Some(10038),
        "G2DA_COLUMN_35" => Some(10039),
        "G2DA_COLUMN_36" => Some(10040),
        "G2DA_COLUMN_37" => Some(10041),
        "G2DA_COLUMN_38" => Some(10042),
        "G2DA_COLUMN_39" => Some(10043),
        "G2DA_COLUMN_4" => Some(10008),
        "G2DA_COLUMN_40" => Some(10044),
        "G2DA_COLUMN_41" => Some(10045),
        "G2DA_COLUMN_42" => Some(10046),
        "G2DA_COLUMN_43" => Some(10047),
        "G2DA_COLUMN_44" => Some(10048),
        "G2DA_COLUMN_45" => Some(10049),
        "G2DA_COLUMN_46" => Some(10050),
        "G2DA_COLUMN_47" => Some(10051),
        "G2DA_COLUMN_48" => Some(10052),
        "G2DA_COLUMN_49" => Some(10053),
        "G2DA_COLUMN_5" => Some(10009),
        "G2DA_COLUMN_50" => Some(10054),
        "G2DA_COLUMN_51" => Some(10055),
        "G2DA_COLUMN_52" => Some(10056),
        "G2DA_COLUMN_53" => Some(10057),
        "G2DA_COLUMN_54" => Some(10058),
        "G2DA_COLUMN_55" => Some(10059),
        "G2DA_COLUMN_56" => Some(10060),
        "G2DA_COLUMN_57" => Some(10061),
        "G2DA_COLUMN_58" => Some(10062),
        "G2DA_COLUMN_59" => Some(10063),
        "G2DA_COLUMN_6" => Some(10010),
        "G2DA_COLUMN_60" => Some(10064),
        "G2DA_COLUMN_61" => Some(10065),
        "G2DA_COLUMN_62" => Some(10066),
        "G2DA_COLUMN_63" => Some(10067),
        "G2DA_COLUMN_64" => Some(10068),
        "G2DA_COLUMN_65" => Some(10069),
        "G2DA_COLUMN_66" => Some(10070),
        "G2DA_COLUMN_67" => Some(10071),
        "G2DA_COLUMN_68" => Some(10072),
        "G2DA_COLUMN_69" => Some(10073),
        "G2DA_COLUMN_7" => Some(10011),
        "G2DA_COLUMN_70" => Some(10074),
        "G2DA_COLUMN_71" => Some(10075),
        "G2DA_COLUMN_72" => Some(10076),
        "G2DA_COLUMN_73" => Some(10077),
        "G2DA_COLUMN_74" => Some(10078),
        "G2DA_COLUMN_75" => Some(10079),
        "G2DA_COLUMN_76" => Some(10080),
        "G2DA_COLUMN_77" => Some(10081),
        "G2DA_COLUMN_78" => Some(10082),
        "G2DA_COLUMN_79" => Some(10083),
        "G2DA_COLUMN_8" => Some(10012),
        "G2DA_COLUMN_80" => Some(10084),
        "G2DA_COLUMN_81" => Some(10085),
        "G2DA_COLUMN_82" => Some(10086),
        "G2DA_COLUMN_83" => Some(10087),
        "G2DA_COLUMN_84" => Some(10088),
        "G2DA_COLUMN_85" => Some(10089),
        "G2DA_COLUMN_86" => Some(10090),
        "G2DA_COLUMN_87" => Some(10091),
        "G2DA_COLUMN_88" => Some(10092),
        "G2DA_COLUMN_89" => Some(10093),
        "G2DA_COLUMN_9" => Some(10013),
        "G2DA_COLUMN_90" => Some(10094),
        "G2DA_COLUMN_91" => Some(10095),
        "G2DA_COLUMN_92" => Some(10096),
        "G2DA_COLUMN_93" => Some(10097),
        "G2DA_COLUMN_94" => Some(10098),
        "G2DA_COLUMN_95" => Some(10099),
        "G2DA_COLUMN_96" => Some(10100),
        "G2DA_COLUMN_97" => Some(10101),
        "G2DA_COLUMN_98" => Some(10102),
        "G2DA_COLUMN_99" => Some(10103),
        "G2DA_COLUMN_HASH" => Some(10001),
        "G2DA_COLUMN_LIST" => Some(10002),
        "G2DA_COLUMN_NAME" => Some(10000),
        "G2DA_COLUMN_TYPE" => Some(10999),
        "G2DA_ROW_DATA" => Some(10004),
        "G2DA_ROW_LIST" => Some(10003),
        "INT16_LIST" => Some(9),
        "INT32_LIST" => Some(11),
        "INT64_LIST" => Some(13),
        "INT8_LIST" => Some(7),
        "INVALIDENTRY" => Some(0),
        "ITEM_ABILITYID" => Some(1011),
        "ITEM_ABILITYPWR" => Some(1012),
        "ITEM_BASECOST" => Some(1021),
        "ITEM_BASEID" => Some(1000),
        "ITEM_CHARGES" => Some(1006),
        "ITEM_COST" => Some(1001),
        "ITEM_CRAFTINGRECIPEID" => Some(1020),
        "ITEM_DESCRIPTION" => Some(1008),
        "ITEM_IDENTIFIED" => Some(1005),
        "ITEM_MATERIAL" => Some(1010),
        "ITEM_MODELVARIATION" => Some(1007),
        "ITEM_PLOT" => Some(1004),
        "ITEM_PROPERTIES" => Some(1013),
        "ITEM_PROPERTYLIST" => Some(1009),
        "ITEM_PROPERTY_EFFECTID" => Some(1015),
        "ITEM_PROPERTY_POWERS" => Some(1014),
        "ITEM_PROPERTY_VFXID" => Some(1018),
        "ITEM_PROP_CHANCEAPPEAR" => Some(2006),
        "ITEM_PROP_COSTTABLE" => Some(2003),
        "ITEM_PROP_COSTVALUE" => Some(2004),
        "ITEM_PROP_PARAM1" => Some(2000),
        "ITEM_PROP_PARAM1VALUE" => Some(2005),
        "ITEM_PROP_PROPERTYNAME" => Some(2001),
        "ITEM_PROP_SUBTYPE" => Some(2002),
        "ITEM_STACKSIZE" => Some(1002),
        "ITEM_STOLEN" => Some(1003),
        "ITEM_SUBITEMS_RESREFS" => Some(1019),
        "LIGHT_AFFECT_DOMAIN" => Some(3079),
        "LIGHT_ANIMATED_MAX_FREQUENCY" => Some(3131),
        "LIGHT_ANIMATED_MAX_INTENSITY" => Some(3133),
        "LIGHT_ANIMATED_MIN_FREQUENCY" => Some(3130),
        "LIGHT_ANIMATED_MIN_INTENSITY" => Some(3132),
        "LIGHT_BAKED" => Some(3077),
        "LIGHT_CAN_BE_OCCLUDED" => Some(3119),
        "LIGHT_COLOR" => Some(3072),
        "LIGHT_COLOR_MULTIPLIER" => Some(3076),
        "LIGHT_EFFECT" => Some(3078),
        "LIGHT_ISDYNAMIC" => Some(3073),
        "LIGHT_POINT_RADIUS" => Some(3075),
        "LIGHT_SPOT_DISTANCE" => Some(3102),
        "LIGHT_SPOT_INNER_ANGLE" => Some(3100),
        "LIGHT_SPOT_OUTER_ANGLE" => Some(3101),
        "LIGHT_TYPE" => Some(3074),
        "LVL_AO_ADAPTSAMPLEACCURACY" => Some(3345),
        "LVL_AO_ADAPTSAMPLEENABLED" => Some(3344),
        "LVL_AO_ADAPTSAMPLESMOOTH" => Some(3346),
        "LVL_AO_COLOR_MAX" => Some(3341),
        "LVL_AO_COLOR_MIN" => Some(3340),
        "LVL_AO_CONEANGLE" => Some(3347),
        "LVL_AO_EXPONENT" => Some(3349),
        "LVL_AO_MAXRAYLENGTH" => Some(3348),
        "LVL_AO_SAMPLES_MAX" => Some(3343),
        "LVL_AO_SAMPLES_MIN" => Some(3342),
        "LVL_CHANGETIME" => Some(3005),
        "LVL_CHILD_LIST" => Some(3003),
        "LVL_COLLISION_WALL_INFO" => Some(3730),
        "LVL_COLLISION_WALL_VERTICIES" => Some(3731),
        "LVL_COLLISION_WALL_VERTICIES_V2" => Some(3732),
        "LVL_FILE_OBJECT_VERSION" => Some(3004),
        "LVL_LIGHTING_VERSION" => Some(3334),
        "LVL_LIGHTMAP_FILESPEC" => Some(3333),
        "LVL_LIGHTMAP_LAST_UPDATED" => Some(3332),
        "LVL_LIGHTMAP_LAST_UPDATED_LIST" => Some(3331),
        "LVL_LIGHTMAP_SIZE_MULTIPLIER" => Some(3330),
        "LVL_LIGHT_SUBSET_ENTRY" => Some(3801),
        "LVL_LIGHT_SUBSET_LIST" => Some(3800),
        "LVL_LIGHT_SUBSET_TOTAL_ENTRIES" => Some(3802),
        "LVL_MINIMAP_POSITION_X" => Some(3740),
        "LVL_MINIMAP_POSITION_Y" => Some(3741),
        "LVL_MINIMAP_SIZE_X" => Some(3742),
        "LVL_MINIMAP_SIZE_Y" => Some(3743),
        "LVL_WATER" => Some(3600),
        "LVL_WATER_COLORIZE_TRANSPARENCY" => Some(3632),
        "LVL_WATER_DEEP_COLOR" => Some(3607),
        "LVL_WATER_ENABLE_SPEC" => Some(3634),
        "LVL_WATER_FOAM_COLOR" => Some(3625),
        "LVL_WATER_FOAM_HEIGHT" => Some(3622),
        "LVL_WATER_HEIGHT_MAP" => Some(3606),
        "LVL_WATER_MAX_TESSELLATION" => Some(3603),
        "LVL_WATER_MESH_ID" => Some(3604),
        "LVL_WATER_NORMAL_MAP" => Some(3605),
        "LVL_WATER_OPACITY_FALLOFF" => Some(3628),
        "LVL_WATER_OVERRIDE_REFLECTION" => Some(3633),
        "LVL_WATER_REFLECTIVITY" => Some(3621),
        "LVL_WATER_SHALLOW_COLOR" => Some(3608),
        "LVL_WATER_SHALLOW_DEPTH" => Some(3624),
        "LVL_WATER_SIZE_X" => Some(3601),
        "LVL_WATER_SIZE_Y" => Some(3602),
        "LVL_WATER_SPECULAR_FALLOFF" => Some(3631),
        "LVL_WATER_SPECULAR_MULTIPLIER" => Some(3630),
        "LVL_WATER_SUBDIVISION_DEPTH_TOLERANCE" => Some(3623),
        "LVL_WATER_SUNLIGHT_SPECULAR_POWER" => Some(3629),
        "LVL_WATER_WALKABLE_DEPTH" => Some(3626),
        "LVL_WATER_WALL_HEIGHT" => Some(3627),
        "LVL_WATER_WAVE_AMPL_1" => Some(3610),
        "LVL_WATER_WAVE_AMPL_2" => Some(3613),
        "LVL_WATER_WAVE_AMPL_3" => Some(3616),
        "LVL_WATER_WAVE_DIRECTION_1" => Some(3611),
        "LVL_WATER_WAVE_DIRECTION_2" => Some(3614),
        "LVL_WATER_WAVE_DIRECTION_3" => Some(3617),
        "LVL_WATER_WAVE_FREQ_1" => Some(3609),
        "LVL_WATER_WAVE_FREQ_2" => Some(3612),
        "LVL_WATER_WAVE_FREQ_3" => Some(3615),
        "LVL_WATER_WAVE_SPEED_1" => Some(3618),
        "LVL_WATER_WAVE_SPEED_2" => Some(3619),
        "LVL_WATER_WAVE_SPEED_3" => Some(3620),
        "LVL_WIND" => Some(3700),
        "LVL_WIND_CLOTH_GUST_AXIS_RATIO" => Some(3729),
        "LVL_WIND_CLOTH_GUST_DIR_CHANGE" => Some(3728),
        "LVL_WIND_CLOTH_GUST_DURATION_MAX" => Some(3725),
        "LVL_WIND_CLOTH_GUST_DURATION_MIN" => Some(3724),
        "LVL_WIND_CLOTH_GUST_INTERVAL_MAX" => Some(3727),
        "LVL_WIND_CLOTH_GUST_INTERVAL_MIN" => Some(3726),
        "LVL_WIND_CLOTH_GUST_STRENGTH_MAX" => Some(3723),
        "LVL_WIND_CLOTH_GUST_STRENGTH_MIN" => Some(3722),
        "LVL_WIND_CLOTH_RESPONSE" => Some(3719),
        "LVL_WIND_CLOTH_RESPONSE_LMT" => Some(3720),
        "LVL_WIND_CLOTH_STRENGTH" => Some(3721),
        "LVL_WIND_ID" => Some(3701),
        "LVL_WIND_ISGLOBAL" => Some(3710),
        "LVL_WIND_NAME" => Some(3702),
        "LVL_WIND_REGIONFALLOFF" => Some(3712),
        "LVL_WIND_REGIONRADIUS" => Some(3711),
        "LVL_WIND_SPTBENDANGLE" => Some(3718),
        "LVL_WIND_SPTGUST_FREQUENCY" => Some(3730),
        "LVL_WIND_SPTGUST_MAXDURATION" => Some(3717),
        "LVL_WIND_SPTGUST_MAXPERCENT" => Some(3715),
        "LVL_WIND_SPTGUST_MINDURATION" => Some(3716),
        "LVL_WIND_SPTGUST_MINPERCENT" => Some(3714),
        "LVL_WIND_SPTSTRENGTH" => Some(3713),
        "MAP_MAPS" => Some(24011),
        "MAP_MAP_PARENT_RESREF" => Some(24012),
        "MAP_PINLIST" => Some(24002),
        "MAP_PIN_AREATAG" => Some(24008),
        "MAP_PIN_NAME" => Some(24006),
        "MAP_PIN_POS_X" => Some(24004),
        "MAP_PIN_POS_Y" => Some(24005),
        "MAP_PIN_STATE" => Some(24003),
        "MAP_PIN_TAG" => Some(24007),
        "MAP_PIN_TERRAINTYPE" => Some(24009),
        "MAP_PIN_TOOLTIP" => Some(24020),
        "MAP_PIN_TYPE" => Some(24010),
        "MAP_PIN_WAYPOINT_OVERRIDE" => Some(24013),
        "MAP_POINTLIST" => Some(24017),
        "MAP_POINT_POS_X" => Some(24018),
        "MAP_POINT_POS_Y" => Some(24019),
        "MAP_TAG" => Some(24000),
        "MAP_TRAILLIST" => Some(24014),
        "MAP_TRAIL_PIN_1_TAG" => Some(24015),
        "MAP_TRAIL_PIN_2_TAG" => Some(24016),
        "MAP_TYPE" => Some(24001),
        "MAT_ALTERNATE_DECAL_COMPRESSION" => Some(15582),
        "MAT_ALTERNATE_DECAL_COMPRESSION_XBOX360" => Some(15583),
        "MAT_ALTERNATE_DECAL_FILENAME" => Some(15581),
        "MAT_ALTERNATE_DECAL_MAP" => Some(15580),
        "MAT_AMBIENT_MULTIPLIER" => Some(15510),
        "MAT_BASIC_PARAMS" => Some(15051),
        "MAT_BLEND_MODE" => Some(15056),
        "MAT_BROW_STUBBLE_COMPRESSION" => Some(15602),
        "MAT_BROW_STUBBLE_COMPRESSION_XBOX360" => Some(15603),
        "MAT_BROW_STUBBLE_FILENAME" => Some(15601),
        "MAT_BROW_STUBBLE_MAP" => Some(15600),
        "MAT_BROW_STUBBLE_NORMAL_COMPRESSION" => Some(15612),
        "MAT_BROW_STUBBLE_NORMAL_COMPRESSION_XBOX360" => Some(15613),
        "MAT_BROW_STUBBLE_NORMAL_FILENAME" => Some(15611),
        "MAT_BROW_STUBBLE_NORMAL_MAP" => Some(15610),
        "MAT_CHILD_LIST" => Some(15001),
        "MAT_DIFFOPAC_COMPRESSION" => Some(15077),
        "MAT_DIFFOPAC_COMPRESSION_XBOX360" => Some(15078),
        "MAT_DIFFOPAC_DIMENSIONX" => Some(15075),
        "MAT_DIFFOPAC_DIMENSIONY" => Some(15076),
        "MAT_DIFFUSE_FILENAME" => Some(15074),
        "MAT_DIFFUSE_MAP" => Some(15073),
        "MAT_DIFFUSE_MAP_COLOR" => Some(15071),
        "MAT_DIFFUSE_MAP_SCALE" => Some(15072),
        "MAT_DIFFUSE_MAP_TYPE" => Some(15070),
        "MAT_DISTORTIONMODIFIERS_COMPRESSION" => Some(15543),
        "MAT_DISTORTIONMODIFIERS_COMPRESSION_XBOX360" => Some(15544),
        "MAT_DISTORTIONMODIFIERS_FILENAME" => Some(15542),
        "MAT_DISTORTIONMODIFIERS_MAP" => Some(15541),
        "MAT_DISTORTIONMODIFIERS_MAP_ENABLE" => Some(15540),
        "MAT_DISTORTION_COMPRESSION" => Some(15523),
        "MAT_DISTORTION_COMPRESSION_XBOX360" => Some(15524),
        "MAT_DISTORTION_FADE_DISTANCE" => Some(15562),
        "MAT_DISTORTION_FADE_MULTIPLIER" => Some(15563),
        "MAT_DISTORTION_FILENAME" => Some(15522),
        "MAT_DISTORTION_INVERT" => Some(15561),
        "MAT_DISTORTION_MAGNITUDE" => Some(15560),
        "MAT_DISTORTION_MAP" => Some(15521),
        "MAT_DISTORTION_MAP_ENABLE" => Some(15520),
        "MAT_DUPLICATE" => Some(15037),
        "MAT_DUPLICATE_NAME" => Some(15038),
        "MAT_DYNC_LIGHT" => Some(15055),
        "MAT_EMISSIVE_COMPRESSION" => Some(15313),
        "MAT_EMISSIVE_COMPRESSION_XBOX360" => Some(15314),
        "MAT_EMISSIVE_FILENAME" => Some(15312),
        "MAT_EMISSIVE_MAP" => Some(15311),
        "MAT_EMISSIVE_MAP_ENABLE" => Some(15310),
        "MAT_EMOTIONS_MASK_0_COMPRESSION" => Some(15622),
        "MAT_EMOTIONS_MASK_0_COMPRESSION_XBOX360" => Some(15623),
        "MAT_EMOTIONS_MASK_0_FILENAME" => Some(15621),
        "MAT_EMOTIONS_MASK_0_MAP" => Some(15620),
        "MAT_EMOTIONS_MASK_1_COMPRESSION" => Some(15632),
        "MAT_EMOTIONS_MASK_1_COMPRESSION_XBOX360" => Some(15633),
        "MAT_EMOTIONS_MASK_1_FILENAME" => Some(15631),
        "MAT_EMOTIONS_MASK_1_MAP" => Some(15630),
        "MAT_EMOTIONS_NORMAL_COMPRESSION" => Some(15642),
        "MAT_EMOTIONS_NORMAL_COMPRESSION_XBOX360" => Some(15643),
        "MAT_EMOTIONS_NORMAL_FILENAME" => Some(15641),
        "MAT_EMOTIONS_NORMAL_MAP" => Some(15640),
        "MAT_EYE_CORNEA_REFLECTION_MULTIPLIER" => Some(15384),
        "MAT_EYE_CORNEA_SPECULAR_MASK" => Some(15380),
        "MAT_EYE_CORNEA_SPECULAR_POWER" => Some(15381),
        "MAT_EYE_SCLERA_SPECULAR_MASK" => Some(15382),
        "MAT_EYE_SCLERA_SPECULAR_POWER" => Some(15383),
        "MAT_FALLOFF_MULTIPLIER" => Some(15503),
        "MAT_FALLOFF_WIDTH" => Some(15502),
        "MAT_FILE_OBJECT_VERSION" => Some(15000),
        "MAT_FRESNEL_COMPRESSION" => Some(15283),
        "MAT_FRESNEL_COMPRESSION_XBOX360" => Some(15284),
        "MAT_FRESNEL_FILENAME" => Some(15282),
        "MAT_FRESNEL_MAP" => Some(15281),
        "MAT_FRESNEL_MAP_ENABLE" => Some(15280),
        "MAT_GROUP" => Some(15027),
        "MAT_GROUP_NAME" => Some(15028),
        "MAT_HAIR" => Some(15054),
        "MAT_HAIR_DIFFUSE_TINT" => Some(15440),
        "MAT_HAIR_PRIMARY_SPECULAR_MASK" => Some(15442),
        "MAT_HAIR_PRIMARY_SPECULAR_POWER" => Some(15441),
        "MAT_HAIR_SECONDARY_SPECULAR_POWER" => Some(15443),
        "MAT_HAIR_SECONDARY_SPECULAR_TINT" => Some(15444),
        "MAT_HAIR_TINT_NOISE_TILING" => Some(15445),
        "MAT_HERALDRYLIB" => Some(15033),
        "MAT_HERALDRYLIB_NAME" => Some(15034),
        "MAT_HERALDRYOBJ" => Some(15035),
        "MAT_HERALDRYOBJ_NAME" => Some(15036),
        "MAT_HERALDRY_COMPRESSION" => Some(15483),
        "MAT_HERALDRY_COMPRESSION_XBOX360" => Some(15484),
        "MAT_HERALDRY_FILENAME" => Some(15482),
        "MAT_HERALDRY_MAP" => Some(15481),
        "MAT_HERALDRY_MAP_ENABLE" => Some(15480),
        "MAT_LAVA_BRIGHTNESS" => Some(15654),
        "MAT_LAVA_CONTRAST" => Some(15655),
        "MAT_LAVA_NOISE_MAP" => Some(15656),
        "MAT_LAVA_TINT_COLOR" => Some(15653),
        "MAT_LAYOUT_NAME" => Some(15039),
        "MAT_LIGHT" => Some(15021),
        "MAT_LIGHT_NAME" => Some(15022),
        "MAT_LIGHT_PROBE" => Some(15025),
        "MAT_LIGHT_PROBE_NAME" => Some(15026),
        "MAT_LIGHT_RIG" => Some(15023),
        "MAT_LIGHT_RIG_NAME" => Some(15024),
        "MAT_LIP_SPECULAR_BOOST" => Some(15512),
        "MAT_MATERIAL_SEMANTIC" => Some(15059),
        "MAT_MATERIAL_SOUND_TYPE" => Some(15060),
        "MAT_MATERIAL_TYPE" => Some(15050),
        "MAT_MATERIAL_TYPE_STRING" => Some(15058),
        "MAT_MATLIB" => Some(15017),
        "MAT_MATLIB_NAME" => Some(15018),
        "MAT_MATOBJ" => Some(15019),
        "MAT_MATOBJ_NAME" => Some(15020),
        "MAT_MODEL" => Some(15012),
        "MAT_MODEL_NAME" => Some(15013),
        "MAT_NAME" => Some(15057),
        "MAT_NORMAL_COMPRESSION" => Some(15163),
        "MAT_NORMAL_COMPRESSION_XBOX360" => Some(15164),
        "MAT_NORMAL_FILENAME" => Some(15162),
        "MAT_NORMAL_MAP" => Some(15161),
        "MAT_NORMAL_MAP_ENABLE" => Some(15160),
        "MAT_OPACITYMAP" => Some(15104),
        "MAT_OPACITYMAPCOLOR" => Some(15102),
        "MAT_OPACITYMAPENABLE" => Some(15100),
        "MAT_OPACITYMAPSCALE" => Some(15103),
        "MAT_OPACITYMAPTYPE" => Some(15101),
        "MAT_PACKED_TEXTURE_COMPRESSION" => Some(15403),
        "MAT_PACKED_TEXTURE_COMPRESSION_XBOX360" => Some(15404),
        "MAT_PACKED_TEXTURE_FILENAME" => Some(15402),
        "MAT_PACKED_TEXTURE_MAP" => Some(15401),
        "MAT_PALETTELIB" => Some(15029),
        "MAT_PALETTELIB_NAME" => Some(15030),
        "MAT_PALETTEOBJ" => Some(15031),
        "MAT_PALETTEOBJ_NAME" => Some(15032),
        "MAT_PART" => Some(15014),
        "MAT_PART_MMH_PARENT" => Some(15016),
        "MAT_PART_NAME" => Some(15015),
        "MAT_RELIEF_COMPRESSION" => Some(15226),
        "MAT_RELIEF_COMPRESSION_XBOX360" => Some(15227),
        "MAT_RELIEF_MAP" => Some(15221),
        "MAT_RELIEF_MAP_ENABLE" => Some(15220),
        "MAT_RELIEF_MAP_IN_OUT" => Some(15225),
        "MAT_RELIEF_MAP_SAMPLES" => Some(15223),
        "MAT_RELIEF_MAP_SCALE" => Some(15222),
        "MAT_RELIEF_MAP_SHADOW_OFFSET" => Some(15224),
        "MAT_RIM_LIGHT_MULTIPLIER" => Some(15501),
        "MAT_RIM_LIGHT_WIDTH" => Some(15500),
        "MAT_RIM_POWER" => Some(15513),
        "MAT_ROOT" => Some(15010),
        "MAT_ROOT_NAME" => Some(15011),
        "MAT_SCROLL_SPEED_1" => Some(15650),
        "MAT_SCROLL_SPEED_2" => Some(15651),
        "MAT_SCROLL_SPEED_3" => Some(15652),
        "MAT_SECONDARY_DIFFUSE_COMPRESSION" => Some(15085),
        "MAT_SECONDARY_DIFFUSE_COMPRESSION_XBOX360" => Some(15086),
        "MAT_SECONDARY_DIFFUSE_FILENAME" => Some(15082),
        "MAT_SECONDARY_DIFFUSE_MAP" => Some(15081),
        "MAT_SECONDARY_DIFFUSE_MAP_ENABLE" => Some(15080),
        "MAT_SECONDARY_NORMAL_COMPRESSION" => Some(15363),
        "MAT_SECONDARY_NORMAL_COMPRESSION_XBOX360" => Some(15364),
        "MAT_SECONDARY_NORMAL_FILENAME" => Some(15362),
        "MAT_SECONDARY_NORMAL_MAP" => Some(15361),
        "MAT_SECONDARY_NORMAL_MAP_ENABLE" => Some(15360),
        "MAT_SECTION_MASK_COMPRESSION" => Some(15343),
        "MAT_SECTION_MASK_COMPRESSION_XBOX360" => Some(15344),
        "MAT_SECTION_MASK_FILENAME" => Some(15342),
        "MAT_SECTION_MASK_MAP" => Some(15341),
        "MAT_SECTION_MASK_MAP_ENABLE" => Some(15340),
        "MAT_SHINY_TRANS" => Some(15052),
        "MAT_SPECULAR_COMPRESSION" => Some(15142),
        "MAT_SPECULAR_COMPRESSION_XBOX360" => Some(15143),
        "MAT_SPECULAR_DIMENSIONX" => Some(15140),
        "MAT_SPECULAR_DIMENSIONY" => Some(15141),
        "MAT_SPECULAR_FILENAME" => Some(15139),
        "MAT_SPECULAR_GLOSS" => Some(15138),
        "MAT_SPECULAR_GLOSS_COLOR" => Some(15136),
        "MAT_SPECULAR_GLOSS_SCALE" => Some(15137),
        "MAT_SPECULAR_GLOSS_TYPE" => Some(15135),
        "MAT_SPECULAR_MAP" => Some(15134),
        "MAT_SPECULAR_MAP_COLOR" => Some(15132),
        "MAT_SPECULAR_MAP_ENABLE" => Some(15130),
        "MAT_SPECULAR_MAP_SCALE" => Some(15133),
        "MAT_SPECULAR_MAP_TYPE" => Some(15131),
        "MAT_SPECULAR_MASK_MAP_ENABLE" => Some(15400),
        "MAT_SPECULAR_MULTIPLIER" => Some(15511),
        "MAT_SPECULAR_REFLECTION_MULTIPLIER" => Some(15144),
        "MAT_SPECULAR_SHIFT_MAP_ENABLE" => Some(15420),
        "MAT_SUN" => Some(15460),
        "MAT_SUN_COLOR" => Some(15463),
        "MAT_SUN_COLORMULT" => Some(15464),
        "MAT_SUN_DIRECTION" => Some(15462),
        "MAT_SUN_NAME" => Some(15461),
        "MAT_TATTOO_MASK_COMPRESSION" => Some(15592),
        "MAT_TATTOO_MASK_COMPRESSION_XBOX360" => Some(15593),
        "MAT_TATTOO_MASK_FILENAME" => Some(15591),
        "MAT_TATTOO_MASK_MAP" => Some(15590),
        "MAT_TATTOO_MASK_MAP_ENABLE" => Some(15594),
        "MAT_TATTOO_MASK_TINT_CHANNEL1" => Some(15595),
        "MAT_TATTOO_MASK_TINT_CHANNEL2" => Some(15596),
        "MAT_TATTOO_MASK_TINT_CHANNEL3" => Some(15597),
        "MAT_TATTOO_MASK_TINT_CHANNEL4" => Some(15598),
        "MAT_TINTLIB" => Some(15040),
        "MAT_TINTLIB_NAME" => Some(15041),
        "MAT_TINTOBJ" => Some(15042),
        "MAT_TINTOBJ_NAME" => Some(15043),
        "MAT_TINT_A_DIFFUSE_INTENSITY" => Some(15206),
        "MAT_TINT_A_DIFFUSE_OPACITY" => Some(15214),
        "MAT_TINT_A_ENABLE" => Some(15198),
        "MAT_TINT_A_SPECULAR_INTENSITY" => Some(15202),
        "MAT_TINT_A_SPECULAR_OPACITY" => Some(15210),
        "MAT_TINT_B_DIFFUSE_INTENSITY" => Some(15205),
        "MAT_TINT_B_DIFFUSE_OPACITY" => Some(15213),
        "MAT_TINT_B_ENABLE" => Some(15194),
        "MAT_TINT_B_SPECULAR_INTENSITY" => Some(15201),
        "MAT_TINT_B_SPECULAR_OPACITY" => Some(15209),
        "MAT_TINT_COMPRESSION" => Some(15196),
        "MAT_TINT_COMPRESSION_XBOX360" => Some(15197),
        "MAT_TINT_EXPORTABLE" => Some(15228),
        "MAT_TINT_FILENAME_POSTFIX" => Some(15195),
        "MAT_TINT_G_DIFFUSE_INTENSITY" => Some(15204),
        "MAT_TINT_G_DIFFUSE_OPACITY" => Some(15212),
        "MAT_TINT_G_ENABLE" => Some(15193),
        "MAT_TINT_G_SPECULAR_INTENSITY" => Some(15200),
        "MAT_TINT_G_SPECULAR_OPACITY" => Some(15208),
        "MAT_TINT_MAP" => Some(15191),
        "MAT_TINT_MAP_ENABLE" => Some(15190),
        "MAT_TINT_MASK_TINT_CHANNEL1" => Some(15216),
        "MAT_TINT_MASK_TINT_CHANNEL2" => Some(15217),
        "MAT_TINT_MASK_TINT_CHANNEL3" => Some(15218),
        "MAT_TINT_MASK_TINT_CHANNEL4" => Some(15219),
        "MAT_TINT_NOISE_COMPRESSION" => Some(15423),
        "MAT_TINT_NOISE_COMPRESSION_XBOX360" => Some(15424),
        "MAT_TINT_NOISE_FILENAME" => Some(15422),
        "MAT_TINT_NOISE_MAP" => Some(15421),
        "MAT_TINT_R_DIFFUSE_INTENSITY" => Some(15203),
        "MAT_TINT_R_DIFFUSE_OPACITY" => Some(15211),
        "MAT_TINT_R_ENABLE" => Some(15192),
        "MAT_TINT_R_SPECULAR_INTENSITY" => Some(15199),
        "MAT_TINT_R_SPECULAR_OPACITY" => Some(15207),
        "MAT_TINT_TYPE" => Some(15215),
        "MAT_TWO_SIDE" => Some(15053),
        "MAT_VFX_CONTACT_SHEET_FRAMES" => Some(15252),
        "MAT_VFX_CONTACT_SHEET_HEIGHT" => Some(15251),
        "MAT_VFX_CONTACT_SHEET_WIDTH" => Some(15250),
        "MAT_VFX_DEPTH_BIAS_ALPHA" => Some(15255),
        "MAT_VFX_END_ALPHA_FRESNEL" => Some(15257),
        "MAT_VFX_INVERT_ALPHA_FRESNEL" => Some(15258),
        "MAT_VFX_SCROLL_SPEED_U" => Some(15253),
        "MAT_VFX_SCROLL_SPEED_V" => Some(15254),
        "MAT_VFX_START_ALPHA_FRESNEL" => Some(15256),
        "MESH_BOUNDS_BOXMAX" => Some(8018),
        "MESH_BOUNDS_BOXMIN" => Some(8017),
        "MESH_BOUNDS_SPHERE" => Some(8019),
        "MESH_CHUNKS" => Some(8021),
        "MESH_CHUNK_ADDITIONALSTREAMS" => Some(8011),
        "MESH_CHUNK_BASEVERTEXINDEX" => Some(8005),
        "MESH_CHUNK_BOUNDS" => Some(8020),
        "MESH_CHUNK_HASINSTGEOM" => Some(8010),
        "MESH_CHUNK_INDEXCOUNT" => Some(8002),
        "MESH_CHUNK_INDEXFORMAT" => Some(8004),
        "MESH_CHUNK_INSTANCES_COUNT" => Some(8034),
        "MESH_CHUNK_MININDEX" => Some(8007),
        "MESH_CHUNK_PRIMITIVETYPE" => Some(8003),
        "MESH_CHUNK_STARTINDEX" => Some(8009),
        "MESH_CHUNK_VERTEXCOUNT" => Some(8001),
        "MESH_CHUNK_VERTEXDECLARATOR" => Some(8025),
        "MESH_CHUNK_VERTEXOFFSET" => Some(8006),
        "MESH_CHUNK_VERTEXSIZE" => Some(8000),
        "MESH_CHUNK_VERTICESREFERENCED" => Some(8008),
        "MESH_INDEXDATA" => Some(8023),
        "MESH_INDEXFORMAT" => Some(8032),
        "MESH_INSTANCED_STREAM" => Some(8033),
        "MESH_STREAM_FREQUENCY" => Some(8014),
        "MESH_STREAM_INSTANCED" => Some(8016),
        "MESH_STREAM_LOOPING" => Some(8015),
        "MESH_STREAM_VERTEXCOUNT" => Some(8013),
        "MESH_STREAM_VERTEXDATA" => Some(8024),
        "MESH_STREAM_VERTEXSIZE" => Some(8012),
        "MESH_VERTEXDATA" => Some(8022),
        "MESH_VERTEXDECLARATOR_DATATYPE" => Some(8028),
        "MESH_VERTEXDECLARATOR_METHOD" => Some(8031),
        "MESH_VERTEXDECLARATOR_OFFSET" => Some(8027),
        "MESH_VERTEXDECLARATOR_STREAM" => Some(8026),
        "MESH_VERTEXDECLARATOR_USAGE" => Some(8029),
        "MESH_VERTEXDECLARATOR_USAGEINDEX" => Some(8030),
        "MMH_ATTRIBUTE_NAME" => Some(6049),
        "MMH_ATTRIBUTE_SOURCE_NAME" => Some(6050),
        "MMH_BONE_INDEX" => Some(6254),
        "MMH_BOUNDING_BOX_MAX" => Some(6055),
        "MMH_BOUNDING_BOX_MIN" => Some(6054),
        "MMH_CHILDREN" => Some(6999),
        "MMH_CLOTH_ATTACHMENT_FLAG_BITFLAGS" => Some(6226),
        "MMH_CLOTH_ATTACHMENT_FLAG_TEARABLE_ATTACHMENT" => Some(6228),
        "MMH_CLOTH_ATTACHMENT_FLAG_TWO_WAY_ATTACHMENT" => Some(6227),
        "MMH_CLOTH_ATTACHMENT_LOCAL_POS" => Some(6231),
        "MMH_CLOTH_ATTACHMENT_RESPONSE_COEFFICIENT" => Some(6204),
        "MMH_CLOTH_ATTACHMENT_SHAPE_NAME" => Some(6229),
        "MMH_CLOTH_ATTACHMENT_TEAR_FACTOR" => Some(6205),
        "MMH_CLOTH_ATTACHMENT_TYPE" => Some(6225),
        "MMH_CLOTH_ATTACHMENT_VERTEX_ID" => Some(6230),
        "MMH_CLOTH_BENDING_STIFFNESS" => Some(6197),
        "MMH_CLOTH_COLLISION_RESPONSE_COEFFICIENT" => Some(6203),
        "MMH_CLOTH_COOKED_DATA_STREAM" => Some(6232),
        "MMH_CLOTH_DAMPING_COEFFICIENT" => Some(6199),
        "MMH_CLOTH_DENSITY" => Some(6196),
        "MMH_CLOTH_EXTERNAL_ACCELERATION" => Some(6207),
        "MMH_CLOTH_FLAG_BENDING" => Some(6217),
        "MMH_CLOTH_FLAG_BENDING_ORTHO" => Some(6218),
        "MMH_CLOTH_FLAG_BITFLAGS" => Some(6210),
        "MMH_CLOTH_FLAG_COLLISION_TWOWAY" => Some(6220),
        "MMH_CLOTH_FLAG_COMDAMPING" => Some(6224),
        "MMH_CLOTH_FLAG_DAMPING" => Some(6219),
        "MMH_CLOTH_FLAG_DISABLE_COLLISION" => Some(6213),
        "MMH_CLOTH_FLAG_GRAVITY" => Some(6216),
        "MMH_CLOTH_FLAG_HARDWARE" => Some(6223),
        "MMH_CLOTH_FLAG_PRESSURE" => Some(6211),
        "MMH_CLOTH_FLAG_SELFCOLLISION" => Some(6214),
        "MMH_CLOTH_FLAG_STATIC" => Some(6212),
        "MMH_CLOTH_FLAG_TEARABLE" => Some(6222),
        "MMH_CLOTH_FLAG_TRIANGLE_COLLISION" => Some(6221),
        "MMH_CLOTH_FLAG_VISUALIZATION" => Some(6215),
        "MMH_CLOTH_FRICTION" => Some(6200),
        "MMH_CLOTH_MESH_GROUP_STRUCT" => Some(6233),
        "MMH_CLOTH_PRESSURE" => Some(6201),
        "MMH_CLOTH_SLEEP_LINEAR_VELOCITY" => Some(6209),
        "MMH_CLOTH_SOLVER_ITERATIONS" => Some(6206),
        "MMH_CLOTH_STRETCHING_STIFFNESS" => Some(6198),
        "MMH_CLOTH_TEAR_FACTOR" => Some(6202),
        "MMH_CLOTH_THICKNESS" => Some(6195),
        "MMH_CLOTH_WAKE_UP_COUNTER" => Some(6208),
        "MMH_CLOTH_WIND_DIRECTION" => Some(6259),
        "MMH_CLOTH_WIND_ENABLED" => Some(6257),
        "MMH_CLOTH_WIND_GUST_AXIS_RATIO" => Some(6270),
        "MMH_CLOTH_WIND_GUST_DIR_CHANGE" => Some(6269),
        "MMH_CLOTH_WIND_GUST_MAX_DURATION" => Some(6266),
        "MMH_CLOTH_WIND_GUST_MAX_INTERVAL" => Some(6268),
        "MMH_CLOTH_WIND_GUST_MAX_STRENGTH" => Some(6264),
        "MMH_CLOTH_WIND_GUST_MIN_DURATION" => Some(6265),
        "MMH_CLOTH_WIND_GUST_MIN_INTERVAL" => Some(6267),
        "MMH_CLOTH_WIND_GUST_MIN_STRENGTH" => Some(6263),
        "MMH_CLOTH_WIND_RESPONSE" => Some(6260),
        "MMH_CLOTH_WIND_RESPONSE_LIMIT" => Some(6261),
        "MMH_CLOTH_WIND_SPACE" => Some(6258),
        "MMH_CLOTH_WIND_SPEEDTREE_DIRECTION" => Some(6273),
        "MMH_CLOTH_WIND_SPEEDTREE_PARAMS" => Some(6276),
        "MMH_CLOTH_WIND_SPEEDTREE_STRENGTH" => Some(6272),
        "MMH_CLOTH_WIND_SPEEDTREE_UPDATE_TIME" => Some(6271),
        "MMH_CLOTH_WIND_STRENGTH" => Some(6262),
        "MMH_COLLISION_GROUP" => Some(6245),
        "MMH_COLLISION_OBJECT_VOLUME" => Some(6236),
        "MMH_DATA_BITFLAGS" => Some(6170),
        "MMH_DATA_COUNT" => Some(6173),
        "MMH_DATA_FREQUENCY" => Some(6175),
        "MMH_DATA_INSTANCED" => Some(6172),
        "MMH_DATA_IS_INDEX_STREAM" => Some(6168),
        "MMH_DATA_LOOPING" => Some(6171),
        "MMH_DATA_PRIMITIVE_TYPE" => Some(6174),
        "MMH_DATA_SEMANTIC" => Some(6167),
        "MMH_DATA_TYPE" => Some(6169),
        "MMH_EMITTER_EMITTER_ATTACHMENT_NAME" => Some(6247),
        "MMH_EMITTER_EMITTER_ATTACHMENT_SPAWN_ON_SURFACE" => Some(6342),
        "MMH_EMITTER_EMITTER_ATTACHMENT_TYPE" => Some(6246),
        "MMH_EMITTER_EMITTER_ATTACHMENT_USE_NORMAL_FOR_VELOCITY" => Some(6343),
        "MMH_EMITTER_FLIPBOOK_TYPE" => Some(6189),
        "MMH_EMITTER_IS_PHYSICS_EMITTER" => Some(6239),
        "MMH_EMITTER_IS_PHYSICS_OBJECT_SPAWN_EMITTER" => Some(6243),
        "MMH_EMITTER_KILL_PARTICLE_WHEN_TARGET_HIT" => Some(6188),
        "MMH_EMITTER_PRESIMULATE_TIME" => Some(6333),
        "MMH_EMITTER_SPAWN_DIRECTION_TRACKS_TARGET" => Some(6187),
        "MMH_EMITTER_TARGET_ATTRACTION" => Some(6185),
        "MMH_EMITTER_TARGET_NAME" => Some(6184),
        "MMH_EMITTER_TARGET_RADIUS" => Some(6186),
        "MMH_EXPORT_CONTROLLER_INDEX" => Some(6274),
        "MMH_EXPORT_CONTROLLER_TYPE" => Some(6053),
        "MMH_EXPORT_EXPORT_NAME" => Some(6052),
        "MMH_EXPORT_TAG_NAME" => Some(6051),
        "MMH_EXPORT_TAG_VARIABLE_TYPE" => Some(6238),
        "MMH_FACIAL_ANIMATION_BLUEPRINT_NAME" => Some(6248),
        "MMH_FLIPBOOK_COLUMNS" => Some(6182),
        "MMH_FLIPBOOK_FRAMES_PER_SECOND" => Some(6180),
        "MMH_FLIPBOOK_RANDOM_START_FRAME" => Some(6183),
        "MMH_FLIPBOOK_ROWS" => Some(6181),
        "MMH_ID" => Some(6004),
        "MMH_JOINT_6DOF_D6_FLAGS" => Some(6166),
        "MMH_JOINT_6DOF_DRIVE_ANGULAR_VELOCITY" => Some(6161),
        "MMH_JOINT_6DOF_DRIVE_LINEAR_VELOCITY" => Some(6160),
        "MMH_JOINT_6DOF_DRIVE_ORIENTATION" => Some(6134),
        "MMH_JOINT_6DOF_DRIVE_POSITION" => Some(6159),
        "MMH_JOINT_6DOF_DRIVE_SLERP_DRIVE_DAMPING" => Some(6157),
        "MMH_JOINT_6DOF_DRIVE_SLERP_DRIVE_FORCE_LIMIT" => Some(6158),
        "MMH_JOINT_6DOF_DRIVE_SLERP_DRIVE_SPRING" => Some(6156),
        "MMH_JOINT_6DOF_DRIVE_SLERP_DRIVE_TYPE" => Some(6155),
        "MMH_JOINT_6DOF_DRIVE_SWING_DRIVE_DAMPING" => Some(6149),
        "MMH_JOINT_6DOF_DRIVE_SWING_DRIVE_FORCE_LIMIT" => Some(6150),
        "MMH_JOINT_6DOF_DRIVE_SWING_DRIVE_SPRING" => Some(6148),
        "MMH_JOINT_6DOF_DRIVE_SWING_DRIVE_TYPE" => Some(6147),
        "MMH_JOINT_6DOF_DRIVE_TWIST_DRIVE_DAMPING" => Some(6153),
        "MMH_JOINT_6DOF_DRIVE_TWIST_DRIVE_FORCE_LIMIT" => Some(6154),
        "MMH_JOINT_6DOF_DRIVE_TWIST_DRIVE_SPRING" => Some(6152),
        "MMH_JOINT_6DOF_DRIVE_TWIST_DRIVE_TYPE" => Some(6151),
        "MMH_JOINT_6DOF_DRIVE_X_DRIVE_DAMPING" => Some(6137),
        "MMH_JOINT_6DOF_DRIVE_X_DRIVE_FORCE_LIMIT" => Some(6138),
        "MMH_JOINT_6DOF_DRIVE_X_DRIVE_SPRING" => Some(6136),
        "MMH_JOINT_6DOF_DRIVE_X_DRIVE_TYPE" => Some(6135),
        "MMH_JOINT_6DOF_DRIVE_Y_DRIVE_DAMPING" => Some(6141),
        "MMH_JOINT_6DOF_DRIVE_Y_DRIVE_FORCE_LIMIT" => Some(6142),
        "MMH_JOINT_6DOF_DRIVE_Y_DRIVE_SPRING" => Some(6140),
        "MMH_JOINT_6DOF_DRIVE_Y_DRIVE_TYPE" => Some(6139),
        "MMH_JOINT_6DOF_DRIVE_Z_DRIVE_DAMPING" => Some(6145),
        "MMH_JOINT_6DOF_DRIVE_Z_DRIVE_FORCE_LIMIT" => Some(6146),
        "MMH_JOINT_6DOF_DRIVE_Z_DRIVE_SPRING" => Some(6144),
        "MMH_JOINT_6DOF_DRIVE_Z_DRIVE_TYPE" => Some(6143),
        "MMH_JOINT_6DOF_GEAR_RATIO" => Some(6164),
        "MMH_JOINT_6DOF_LINEAR_LIMIT" => Some(6129),
        "MMH_JOINT_6DOF_PROJECTION_ANGLE" => Some(6163),
        "MMH_JOINT_6DOF_PROJECTION_DISTANCE" => Some(6162),
        "MMH_JOINT_6DOF_PROJECTION_MODE" => Some(6165),
        "MMH_JOINT_6DOF_SWING_1_LIMIT" => Some(6130),
        "MMH_JOINT_6DOF_SWING_1_MOTION" => Some(6126),
        "MMH_JOINT_6DOF_SWING_2_LIMIT" => Some(6131),
        "MMH_JOINT_6DOF_SWING_2_MOTION" => Some(6127),
        "MMH_JOINT_6DOF_TWIST_LIMIT_HIGH" => Some(6133),
        "MMH_JOINT_6DOF_TWIST_LIMIT_LOW" => Some(6132),
        "MMH_JOINT_6DOF_TWIST_MOTION" => Some(6128),
        "MMH_JOINT_6DOF_X_MOTION" => Some(6123),
        "MMH_JOINT_6DOF_Y_MOTION" => Some(6124),
        "MMH_JOINT_6DOF_Z_MOTION" => Some(6125),
        "MMH_JOINT_COLLISION_ENABLED" => Some(6089),
        "MMH_JOINT_DISTANCE_DISTANCE_FLAGS" => Some(6113),
        "MMH_JOINT_DISTANCE_MAX_DISTANCE" => Some(6111),
        "MMH_JOINT_DISTANCE_MIN_DISTANCE" => Some(6110),
        "MMH_JOINT_DISTANCE_SPRING" => Some(6112),
        "MMH_JOINT_LOCAL_ANCHOR_1" => Some(6083),
        "MMH_JOINT_LOCAL_ANCHOR_2" => Some(6084),
        "MMH_JOINT_LOCAL_AXIS_1" => Some(6085),
        "MMH_JOINT_LOCAL_AXIS_2" => Some(6086),
        "MMH_JOINT_LOCAL_NORMAL_1" => Some(6081),
        "MMH_JOINT_LOCAL_NORMAL_2" => Some(6082),
        "MMH_JOINT_MAX_FORCE" => Some(6087),
        "MMH_JOINT_MAX_TORQUE" => Some(6088),
        "MMH_JOINT_PULLEY_DISTANCE" => Some(6116),
        "MMH_JOINT_PULLEY_MOTOR_FREE_SPIN" => Some(6121),
        "MMH_JOINT_PULLEY_MOTOR_MAX_FORCE" => Some(6120),
        "MMH_JOINT_PULLEY_MOTOR_VEL_TARGET" => Some(6119),
        "MMH_JOINT_PULLEY_PULLEY_1" => Some(6114),
        "MMH_JOINT_PULLEY_PULLEY_2" => Some(6115),
        "MMH_JOINT_PULLEY_PULLEY_FLAGS" => Some(6122),
        "MMH_JOINT_PULLEY_RATIO" => Some(6118),
        "MMH_JOINT_PULLEY_STIFFNESS" => Some(6117),
        "MMH_JOINT_REVOLUTE_LIMIT_HIGH" => Some(6101),
        "MMH_JOINT_REVOLUTE_LIMIT_LOW" => Some(6100),
        "MMH_JOINT_REVOLUTE_MOTOR_FREE_SPIN" => Some(6108),
        "MMH_JOINT_REVOLUTE_MOTOR_MAX_FORCE" => Some(6107),
        "MMH_JOINT_REVOLUTE_MOTOR_VEL_TARGET" => Some(6106),
        "MMH_JOINT_REVOLUTE_PROJECTION_ANGLE" => Some(6103),
        "MMH_JOINT_REVOLUTE_PROJECTION_DISTANCE" => Some(6102),
        "MMH_JOINT_REVOLUTE_PROJECTION_MODE" => Some(6104),
        "MMH_JOINT_REVOLUTE_REVOLUTE_FLAGS" => Some(6109),
        "MMH_JOINT_REVOLUTE_SPRING" => Some(6105),
        "MMH_JOINT_SPHERICAL_JOINT_SPRING" => Some(6097),
        "MMH_JOINT_SPHERICAL_PROJECTION_DISTANCE" => Some(6091),
        "MMH_JOINT_SPHERICAL_PROJECTION_MODE" => Some(6098),
        "MMH_JOINT_SPHERICAL_SPHERE_FLAGS" => Some(6099),
        "MMH_JOINT_SPHERICAL_SWING_AXIS" => Some(6090),
        "MMH_JOINT_SPHERICAL_SWING_LIMIT" => Some(6094),
        "MMH_JOINT_SPHERICAL_SWING_SPRING" => Some(6096),
        "MMH_JOINT_SPHERICAL_TWIST_LIMIT_HIGH" => Some(6093),
        "MMH_JOINT_SPHERICAL_TWIST_LIMIT_LOW" => Some(6092),
        "MMH_JOINT_SPHERICAL_TWIST_SWING" => Some(6095),
        "MMH_JOINT_TARGET" => Some(6078),
        "MMH_JOINT_TYPE" => Some(6079),
        "MMH_JOINT_TYPE_STRUCT" => Some(6080),
        "MMH_LIGHTPROBE_IRRADIANCE_BLUE" => Some(6338),
        "MMH_LIGHTPROBE_IRRADIANCE_COEFFICIENTS_BLUE" => Some(6192),
        "MMH_LIGHTPROBE_IRRADIANCE_COEFFICIENTS_GREEN" => Some(6191),
        "MMH_LIGHTPROBE_IRRADIANCE_COEFFICIENTS_RED" => Some(6190),
        "MMH_LIGHTPROBE_IRRADIANCE_GREEN" => Some(6337),
        "MMH_LIGHTPROBE_IRRADIANCE_RED" => Some(6336),
        "MMH_LIGHTPROBE_IRRADIANCE_RES" => Some(6253),
        "MMH_LIGHT_CAN_BE_OCCLUDED" => Some(6339),
        "MMH_MATERIAL_LIBRARY" => Some(6002),
        "MMH_MATERIAL_OBJECT" => Some(6001),
        "MMH_MESH_BONES_USED" => Some(6255),
        "MMH_MESH_CAST_BAKED_SHADOW" => Some(6177),
        "MMH_MESH_CAST_RUNTIME_SHADOW" => Some(6176),
        "MMH_MESH_CUT_AWAY" => Some(6193),
        "MMH_MESH_DEFAULT_HIDDEN" => Some(6346),
        "MMH_MESH_GROUP_NAME" => Some(6006),
        "MMH_MESH_IS_VFX_MESH" => Some(6334),
        "MMH_MESH_MATERIAL_COLOR" => Some(6335),
        "MMH_MESH_PUNCH_THROUGH" => Some(6194),
        "MMH_MESH_RECEIVE_BAKED_SHADOW" => Some(6301),
        "MMH_MESH_RECEIVE_RUNTIME_SHADOW" => Some(6304),
        "MMH_MODEL_HIERARCHY_MODEL_DATA_NAME" => Some(6005),
        "MMH_MODEL_MESH_NAME_LIST" => Some(6306),
        "MMH_NAME" => Some(6000),
        "MMH_NODE_AGE_MAP_COUNT" => Some(6039),
        "MMH_NODE_AGE_MAP_ELEMENT_COLOR" => Some(6043),
        "MMH_NODE_AGE_MAP_ELEMENT_PERCENT_LIFE_ELAPSED" => Some(6040),
        "MMH_NODE_AGE_MAP_ELEMENT_SCALE_X" => Some(6041),
        "MMH_NODE_AGE_MAP_ELEMENT_SCALE_Y" => Some(6042),
        "MMH_NODE_AMBIENT_LIGHT_COLOR" => Some(6010),
        "MMH_NODE_COLLISION_OBJ_DENSITY" => Some(6056),
        "MMH_NODE_COLLISION_OBJ_TYPE" => Some(6057),
        "MMH_NODE_CRUST_HOOK_ID" => Some(6235),
        "MMH_NODE_EMITTER_ACCELERATION" => Some(6018),
        "MMH_NODE_EMITTER_AGEMAP_COLOR_MULTIPLIER" => Some(6279),
        "MMH_NODE_EMITTER_AGEMAP_SCALEX_MULTIPLIER" => Some(6280),
        "MMH_NODE_EMITTER_AGEMAP_SCALEY_MULTIPLIER" => Some(6281),
        "MMH_NODE_EMITTER_BIRTH_RATE" => Some(6011),
        "MMH_NODE_EMITTER_BIRTH_RATE_RANGE" => Some(6012),
        "MMH_NODE_EMITTER_CAN_PARTICLES_SPLAT" => Some(6321),
        "MMH_NODE_EMITTER_GRAVITY_MULTIPLIER" => Some(6031),
        "MMH_NODE_EMITTER_IGNORE_DISTORTION" => Some(6309),
        "MMH_NODE_EMITTER_INITIAL_ROTATION" => Some(6299),
        "MMH_NODE_EMITTER_INITIAL_ROTATION_RANGE" => Some(6300),
        "MMH_NODE_EMITTER_INITIAL_ROTATION_SPEED" => Some(6019),
        "MMH_NODE_EMITTER_INITIAL_ROTATION_SPEED_RANGE" => Some(6020),
        "MMH_NODE_EMITTER_INITIAL_SPEED" => Some(6016),
        "MMH_NODE_EMITTER_INITIAL_SPEED_RANGE" => Some(6017),
        "MMH_NODE_EMITTER_LIFE" => Some(6013),
        "MMH_NODE_EMITTER_LIFE_RANGE" => Some(6014),
        "MMH_NODE_EMITTER_LOD" => Some(6323),
        "MMH_NODE_EMITTER_MESH_PARTICLE_MODELNAME" => Some(6284),
        "MMH_NODE_EMITTER_MESH_PARTICLE_ROLL_AXIS" => Some(6303),
        "MMH_NODE_EMITTER_MESH_PARTICLE_UP_AXIS" => Some(6302),
        "MMH_NODE_EMITTER_MOVEMENT_SPREAD_X" => Some(6025),
        "MMH_NODE_EMITTER_MOVEMENT_SPREAD_Y" => Some(6026),
        "MMH_NODE_EMITTER_OPTIONS_BIRTHRATE_IN_PARTICLES_PER_METER" => Some(6028),
        "MMH_NODE_EMITTER_OPTIONS_BITFLAGS" => Some(6027),
        "MMH_NODE_EMITTER_OPTIONS_BOUNCINESS" => Some(6282),
        "MMH_NODE_EMITTER_OPTIONS_ENABLE_PARTICLE_COLLISIONS" => Some(6035),
        "MMH_NODE_EMITTER_OPTIONS_FRICTION" => Some(6283),
        "MMH_NODE_EMITTER_OPTIONS_INHERIT_VELOCITY_INSTEAD_OF_POSITION" => Some(6036),
        "MMH_NODE_EMITTER_OPTIONS_LINK_PARTICLES_TOGETHER" => Some(6033),
        "MMH_NODE_EMITTER_OPTIONS_OBJECT_SPACE_ACCELERATION" => Some(6298),
        "MMH_NODE_EMITTER_OPTIONS_PARTICLES_AFFECTED_BY_WIND" => Some(6030),
        "MMH_NODE_EMITTER_OPTIONS_PARTICLES_FOLLOW_PATH" => Some(6032),
        "MMH_NODE_EMITTER_OPTIONS_RANDOM_INITIAL_ROTATION" => Some(6029),
        "MMH_NODE_EMITTER_OPTIONS_UPDATE_ONLY_WHEN_VISIBLE" => Some(6034),
        "MMH_NODE_EMITTER_ORIENTATION_BEHAVIOR" => Some(6037),
        "MMH_NODE_EMITTER_PARTICLE_INHERITANCE" => Some(6038),
        "MMH_NODE_EMITTER_ROTATIONAL_ACCELERATION" => Some(6021),
        "MMH_NODE_EMITTER_SCALE_RANGE" => Some(6015),
        "MMH_NODE_EMITTER_SPAWN_SPREAD_X" => Some(6023),
        "MMH_NODE_EMITTER_SPAWN_SPREAD_Y" => Some(6024),
        "MMH_NODE_EMITTER_SPLATPARAMS_AGEMAP_COLOR_MULTIPLIER" => Some(6344),
        "MMH_NODE_EMITTER_SPLATPARAMS_AGE_MAP_ELEMENT_PERCENT_LIFE_ELAPSED" => Some(6322),
        "MMH_NODE_EMITTER_SPLATPARAMS_FLIPBOOK_COLUMNS" => Some(6319),
        "MMH_NODE_EMITTER_SPLATPARAMS_FLIPBOOK_FRAMES_PER_SECOND" => Some(6317),
        "MMH_NODE_EMITTER_SPLATPARAMS_FLIPBOOK_RANDOM_START_FRAME" => Some(6320),
        "MMH_NODE_EMITTER_SPLATPARAMS_FLIPBOOK_ROWS" => Some(6318),
        "MMH_NODE_EMITTER_SPLATPARAMS_FLIPBOOK_TYPE" => Some(6316),
        "MMH_NODE_EMITTER_SPLATPARAMS_HEIGHT" => Some(6311),
        "MMH_NODE_EMITTER_SPLATPARAMS_HOLD_LAST_FRAME" => Some(6341),
        "MMH_NODE_EMITTER_SPLATPARAMS_LIFE" => Some(6315),
        "MMH_NODE_EMITTER_SPLATPARAMS_MATERIALNAME" => Some(6324),
        "MMH_NODE_EMITTER_SPLATPARAMS_NUMSAMPLES_HEIGHT" => Some(6313),
        "MMH_NODE_EMITTER_SPLATPARAMS_NUMSAMPLES_WIDTH" => Some(6312),
        "MMH_NODE_EMITTER_SPLATPARAMS_ORIENTATION_RANGE" => Some(6314),
        "MMH_NODE_EMITTER_SPLATPARAMS_WIDTH" => Some(6310),
        "MMH_NODE_EMITTER_TYPE" => Some(6234),
        "MMH_NODE_EMITTER_USER_PARAM_NAME" => Some(6325),
        "MMH_NODE_EMITTER_UV_DISTRIBUTION_SIZE" => Some(6308),
        "MMH_NODE_EMITTER_WORLD_AXIS_ACCELERATION" => Some(6294),
        "MMH_NODE_INV_EMITTER_MOVEMENT_SPREAD_UPDATE_DELAY" => Some(6022),
        "MMH_NODE_LIGHT_AFFECT_DOMAIN" => Some(6296),
        "MMH_NODE_LIGHT_VERSION" => Some(6345),
        "MMH_NODE_MESH_NAME" => Some(6307),
        "MMH_NODE_POINT_LIGHT_COLOR" => Some(6007),
        "MMH_NODE_POINT_LIGHT_INTENSITY_PERIOD" => Some(6250),
        "MMH_NODE_POINT_LIGHT_INTENSITY_PERIOD_DELTA" => Some(6251),
        "MMH_NODE_POINT_LIGHT_INTENSITY_VARIATION" => Some(6249),
        "MMH_NODE_POINT_LIGHT_IS_STATIC" => Some(6009),
        "MMH_NODE_POINT_LIGHT_RADIUS" => Some(6008),
        "MMH_NODE_SOUND_MATERIAL" => Some(6330),
        "MMH_NODE_SPAWN_VOLUME_BOX_MAX" => Some(6290),
        "MMH_NODE_SPAWN_VOLUME_BOX_MIN" => Some(6289),
        "MMH_NODE_SPAWN_VOLUME_CYLINDER_AXIS" => Some(6288),
        "MMH_NODE_SPAWN_VOLUME_CYLINDER_LENGTH" => Some(6287),
        "MMH_NODE_SPAWN_VOLUME_OPTIONS_BITFLAGS" => Some(6044),
        "MMH_NODE_SPAWN_VOLUME_OPTIONS_INVERT_SPAWN_VOLUME_NORMALS" => Some(6046),
        "MMH_NODE_SPAWN_VOLUME_OPTIONS_NORMALS_AS_DIRECTION" => Some(6291),
        "MMH_NODE_SPAWN_VOLUME_OPTIONS_SPAWN_WITHIN_VOLUME" => Some(6045),
        "MMH_NODE_SPAWN_VOLUME_RADIUS" => Some(6286),
        "MMH_NODE_SPAWN_VOLUME_TYPE" => Some(6285),
        "MMH_OBJECT_VOLUME" => Some(6237),
        "MMH_REMOTE_MATERIAL_ALPHA" => Some(6331),
        "MMH_REMOTE_MATERIAL_DECAL_NAME" => Some(6327),
        "MMH_REMOTE_MATERIAL_FRESNEL_FALLOFF" => Some(6328),
        "MMH_REMOTE_MATERIAL_INVERT_FRESNEL" => Some(6329),
        "MMH_REMOTE_MATERIAL_TINT" => Some(6332),
        "MMH_RESNAME" => Some(6003),
        "MMH_ROTATION" => Some(6048),
        "MMH_SCALE" => Some(6278),
        "MMH_SHAPE_ALLOW_EMITTER_SPAWN" => Some(6244),
        "MMH_SHAPE_BOX_DIM" => Some(6071),
        "MMH_SHAPE_COLLISION_MASK_ALL" => Some(6063),
        "MMH_SHAPE_COLLISION_MASK_BITFLAGS" => Some(6062),
        "MMH_SHAPE_COLLISION_MASK_CREATURES" => Some(6066),
        "MMH_SHAPE_COLLISION_MASK_EFFECTS" => Some(6178),
        "MMH_SHAPE_COLLISION_MASK_ITEMS" => Some(6065),
        "MMH_SHAPE_COLLISION_MASK_NONE" => Some(6064),
        "MMH_SHAPE_COLLISION_MASK_NONWALKABLE" => Some(6070),
        "MMH_SHAPE_COLLISION_MASK_PLACEABLES" => Some(6067),
        "MMH_SHAPE_COLLISION_MASK_STATIC_GEOMETRY" => Some(6069),
        "MMH_SHAPE_COLLISION_MASK_TERRAIN_WALL" => Some(6295),
        "MMH_SHAPE_COLLISION_MASK_TRIGGERS" => Some(6068),
        "MMH_SHAPE_COLLISION_MASK_WALKABLE" => Some(6305),
        "MMH_SHAPE_COLLISION_MASK_WATER" => Some(6277),
        "MMH_SHAPE_COLLISION_MASK_WAYPOINTS" => Some(6179),
        "MMH_SHAPE_COOKED_DATA_STREAM" => Some(6077),
        "MMH_SHAPE_FADEABLE" => Some(6252),
        "MMH_SHAPE_HEIGHT" => Some(6073),
        "MMH_SHAPE_MESH_HEIGHT_FIELD_AXIS" => Some(6075),
        "MMH_SHAPE_MESH_HEIGHT_FIELD_EXTENT" => Some(6076),
        "MMH_SHAPE_MESH_SHAPE_FLAGS" => Some(6074),
        "MMH_SHAPE_NAME" => Some(6241),
        "MMH_SHAPE_PMAT_NAME" => Some(6059),
        "MMH_SHAPE_POSITION" => Some(6061),
        "MMH_SHAPE_RADIUS" => Some(6072),
        "MMH_SHAPE_ROTATION" => Some(6060),
        "MMH_SHAPE_TYPE" => Some(6058),
        "MMH_SHAPE_TYPE_STRUCT" => Some(6998),
        "MMH_SHAPE_VOLUME" => Some(6240),
        "MMH_SNAP_POSITION" => Some(6242),
        "MMH_TOTAL_BONES" => Some(6256),
        "MMH_TOTAL_EXPORTS" => Some(6275),
        "MMH_TRANSLATION" => Some(6047),
        "MMH_USE_VARIATION_TINT" => Some(6340),
        "MMH_WEAPONTRAIL_DURATION" => Some(6293),
        "MMH_WEAPONTRAIL_SEGMENT_LENGTH" => Some(6292),
        "MMN_NODE_EMITTER_VERTEX_FORMAT" => Some(6297),
        "MOON_ALPHA" => Some(22701),
        "MOON_CLOUDALPHA" => Some(22702),
        "MOON_ROTATION" => Some(22703),
        "MOON_SCALE" => Some(22700),
        "MORPH_FLOATPARAM" => Some(23006),
        "MORPH_FLOATPARAMVALUE" => Some(23007),
        "MORPH_MAT_NODE_NAME" => Some(23009),
        "MORPH_MAT_PARAMS" => Some(23014),
        "MORPH_MAT_PARAM_INDEX" => Some(23011),
        "MORPH_MAT_PARAM_NAME" => Some(23010),
        "MORPH_MAT_PARAM_VALUE" => Some(23012),
        "MORPH_MAT_PARAM_VECTOR" => Some(23013),
        "MORPH_MAT_VEC_PARAMS" => Some(23015),
        "MORPH_MODEL_NAME" => Some(23016),
        "MORPH_MODEL_PARAMS" => Some(23018),
        "MORPH_MODEL_VALUE" => Some(23017),
        "MORPH_NAME" => Some(23008),
        "MORPH_NODES" => Some(23002),
        "MORPH_PARTS" => Some(23000),
        "MORPH_TEXTUREPARAM" => Some(23004),
        "MORPH_TEXTURES" => Some(23022),
        "MORPH_TEXTURE_NAME" => Some(23003),
        "MORPH_TEX_NAME" => Some(23021),
        "MORPH_TEX_NODE_NAME" => Some(23019),
        "MORPH_TEX_PARAM_NAME" => Some(23020),
        "MORPH_TINTFILENAMES" => Some(23001),
        "MORPH_VECTOR4FPARAM" => Some(23005),
        "NAME" => Some(2),
        "NAME_HASH" => Some(21),
        "OBJECT_ID" => Some(23),
        "ORIENTATION" => Some(5),
        "PLACEABLE_STATES_LIST" => Some(20000),
        "PLOTASSIST_ADVANCES_PLOT" => Some(13021),
        "PLOTASSIST_LIST" => Some(13019),
        "PLOTASSIST_TAG" => Some(13020),
        "PLOT_ALLOW_PAUSING" => Some(13023),
        "PLOT_ENTRYTYPE" => Some(13022),
        "PLOT_FLAGS" => Some(13000),
        "PLOT_FLAGS1" => Some(13011),
        "PLOT_FLAGS2" => Some(13012),
        "PLOT_FLAGS3" => Some(13013),
        "PLOT_FLAGS4" => Some(13014),
        "PLOT_FLAG_AREA_LOCATION_TAG" => Some(13018),
        "PLOT_FLAG_ENDS_PLOT" => Some(13005),
        "PLOT_FLAG_ID" => Some(13001),
        "PLOT_FLAG_JOURNAL" => Some(13004),
        "PLOT_FLAG_MULTIREWARD" => Some(13006),
        "PLOT_FLAG_NAME" => Some(13002),
        "PLOT_FLAG_OFFERID" => Some(13024),
        "PLOT_FLAG_REWARD" => Some(13003),
        "PLOT_GUID" => Some(13007),
        "PLOT_JOURNAL_IMAGE" => Some(13015),
        "PLOT_NAME" => Some(13008),
        "PLOT_PARENT_PLOT" => Some(13017),
        "PLOT_PARENT_PLOT_GUID" => Some(13025),
        "PLOT_PLOTS" => Some(13016),
        "PLOT_PRIORITY" => Some(13010),
        "PLOT_SCRIPT" => Some(13009),
        "POSITION" => Some(4),
        "QUATERNIONF_LIST" => Some(18),
        "RIMTREE_CHILD_LIST" => Some(3292),
        "RIMTREE_NODE_RESREF" => Some(3294),
        "RIMTREE_NODE_TAG" => Some(3293),
        "RIMTREE_RIM_LIST" => Some(3291),
        "RIMTREE_ROOT_NODE" => Some(3290),
        "SAVEGAME_ABILITYLIST" => Some(16309),
        "SAVEGAME_ADDINSLIST" => Some(16006),
        "SAVEGAME_ADDIN_CSCZ" => Some(16429),
        "SAVEGAME_ADDIN_DEDE" => Some(16424),
        "SAVEGAME_ADDIN_ENUS" => Some(16421),
        "SAVEGAME_ADDIN_ESES" => Some(16425),
        "SAVEGAME_ADDIN_FRFR" => Some(16422),
        "SAVEGAME_ADDIN_HUHU" => Some(16430),
        "SAVEGAME_ADDIN_ITIT" => Some(16423),
        "SAVEGAME_ADDIN_NAME" => Some(16960),
        "SAVEGAME_ADDIN_PLPL" => Some(16426),
        "SAVEGAME_ADDIN_PSEUDO" => Some(16428),
        "SAVEGAME_ADDIN_RURU" => Some(16427),
        "SAVEGAME_ADDIN_UID" => Some(16420),
        "SAVEGAME_AI_MASTER" => Some(16636),
        "SAVEGAME_AMBIENTDIALOG_LINE" => Some(16534),
        "SAVEGAME_AMBIENTDIALOG_LIST" => Some(16530),
        "SAVEGAME_AMBIENTDIALOG_OWNER" => Some(16531),
        "SAVEGAME_AMBIENTDIALOG_RESREF" => Some(16533),
        "SAVEGAME_AMBIENTDIALOG_SPEAKER" => Some(16532),
        "SAVEGAME_AOE_ABILITY_ID" => Some(16750),
        "SAVEGAME_AOE_CREATOR" => Some(16608),
        "SAVEGAME_AOE_DURATION" => Some(16609),
        "SAVEGAME_AOE_DURATION_TYPE" => Some(16610),
        "SAVEGAME_AOE_FLAGS" => Some(16751),
        "SAVEGAME_AOE_ID" => Some(16603),
        "SAVEGAME_AOE_LENGTH" => Some(16607),
        "SAVEGAME_AOE_LINKED" => Some(16611),
        "SAVEGAME_AOE_RADIUS" => Some(16605),
        "SAVEGAME_AOE_SHAPE" => Some(16604),
        "SAVEGAME_AOE_STATIONARY" => Some(16752),
        "SAVEGAME_AOE_WIDTH" => Some(16606),
        "SAVEGAME_APPEARANCE" => Some(16320),
        "SAVEGAME_APPEARANCE_DECAPITATED" => Some(16325),
        "SAVEGAME_APPEARANCE_GENDER" => Some(16322),
        "SAVEGAME_APPEARANCE_GORE" => Some(16324),
        "SAVEGAME_APPEARANCE_ITEM_HERALDRY_VARIATION" => Some(16326),
        "SAVEGAME_APPEARANCE_MORPH_NAME" => Some(16328),
        "SAVEGAME_APPEARANCE_ORIGINAL_TYPE" => Some(16327),
        "SAVEGAME_APPEARANCE_TYPE" => Some(16321),
        "SAVEGAME_AREALIST" => Some(16001),
        "SAVEGAME_AREA_AOES" => Some(16013),
        "SAVEGAME_AREA_CREATURES" => Some(16011),
        "SAVEGAME_AREA_MAP" => Some(16016),
        "SAVEGAME_AREA_MIN_CREATURE_IMPORTANCE" => Some(16020),
        "SAVEGAME_AREA_PLACEABLES" => Some(16010),
        "SAVEGAME_AREA_PLACEABLE_STATE" => Some(16100),
        "SAVEGAME_AREA_PLACEABLE_USEABLE" => Some(16102),
        "SAVEGAME_AREA_ROOMS_VIEWED" => Some(16018),
        "SAVEGAME_AREA_SOUNDS" => Some(16019),
        "SAVEGAME_AREA_STORES" => Some(16017),
        "SAVEGAME_AREA_TRIGGERS" => Some(16012),
        "SAVEGAME_AREA_TRIGGER_DCDETECTCHECK" => Some(16105),
        "SAVEGAME_AREA_TRIGGER_DCDISARMCHECK" => Some(16106),
        "SAVEGAME_AREA_TRIGGER_DETECTABLE" => Some(16103),
        "SAVEGAME_AREA_TRIGGER_DISARMABLE" => Some(16104),
        "SAVEGAME_AREA_TRIGGER_GEOMETRY" => Some(16101),
        "SAVEGAME_AREA_TRIGGER_LAST_DISARMED" => Some(16107),
        "SAVEGAME_AREA_TRIGGER_LOAD_SCREEN" => Some(16110),
        "SAVEGAME_AREA_TRIGGER_MUSICVOLUME_ENTERSTATE" => Some(16113),
        "SAVEGAME_AREA_TRIGGER_MUSICVOLUME_ENTERSTATEDELAY" => Some(16115),
        "SAVEGAME_AREA_TRIGGER_MUSICVOLUME_EXITSTATE" => Some(16114),
        "SAVEGAME_AREA_TRIGGER_MUSICVOLUME_EXITSTATEDELAY" => Some(16116),
        "SAVEGAME_AREA_TRIGGER_PRIORITY" => Some(16109),
        "SAVEGAME_AREA_TRIGGER_REVERB_PRESET" => Some(16108),
        "SAVEGAME_AREA_TRIGGER_SOUNDS" => Some(16111),
        "SAVEGAME_AREA_TRIGGER_TYPE" => Some(16112),
        "SAVEGAME_AREA_WAYPOINTS" => Some(16015),
        "SAVEGAME_AUTOLEVELUP" => Some(16329),
        "SAVEGAME_BACKPACK" => Some(16210),
        "SAVEGAME_BODYBAG_ID" => Some(16600),
        "SAVEGAME_BUILD_NUMBER" => Some(16770),
        "SAVEGAME_CAMPAIGN" => Some(16000),
        "SAVEGAME_CAMPAIGN_RESOURCE" => Some(16014),
        "SAVEGAME_CAN_LEVELUP" => Some(16456),
        "SAVEGAME_CHEAT_USED" => Some(16007),
        "SAVEGAME_COMMAND_COMMANDID" => Some(16722),
        "SAVEGAME_COMMAND_DATA" => Some(16725),
        "SAVEGAME_COMMAND_ID" => Some(16723),
        "SAVEGAME_COMMAND_LIST" => Some(16721),
        "SAVEGAME_COMMAND_PLAYERISSUED" => Some(16726),
        "SAVEGAME_COMMAND_STATIC" => Some(16724),
        "SAVEGAME_CRAFTING_RECIPE_LIST" => Some(16227),
        "SAVEGAME_CREATURE_ABILITY_HEADER_ID" => Some(16473),
        "SAVEGAME_CREATURE_CANCHANGEEQUIPMENT" => Some(16463),
        "SAVEGAME_CREATURE_CLASS_ID" => Some(16465),
        "SAVEGAME_CREATURE_CLASS_RANK" => Some(16466),
        "SAVEGAME_CREATURE_CLASS_RANK_LIST" => Some(16464),
        "SAVEGAME_CREATURE_CONTROLLABLE" => Some(16458),
        "SAVEGAME_CREATURE_HEATBEAT_INTERVAL" => Some(16475),
        "SAVEGAME_CREATURE_INTERACTIVE" => Some(16459),
        "SAVEGAME_CREATURE_IS_GHOST" => Some(16467),
        "SAVEGAME_CREATURE_IS_STATUE" => Some(16470),
        "SAVEGAME_CREATURE_ITEMS_SCALED" => Some(16474),
        "SAVEGAME_CREATURE_MINIMIZED_SKILL_HEADER_LIST" => Some(16471),
        "SAVEGAME_CREATURE_MINIMIZED_TALENT_HEADER_LIST" => Some(16472),
        "SAVEGAME_CREATURE_MODAL_ABILITY_LIST" => Some(16468),
        "SAVEGAME_CREATURE_NOPERMDEATH" => Some(16480),
        "SAVEGAME_CREATURE_PACKAGE" => Some(16461),
        "SAVEGAME_CREATURE_PACKAGE_AI" => Some(16462),
        "SAVEGAME_CREATURE_POOL_AVAILABLE" => Some(16479),
        "SAVEGAME_CREATURE_POOL_NAME" => Some(16478),
        "SAVEGAME_CREATURE_RACE" => Some(16460),
        "SAVEGAME_CREATURE_RANK" => Some(16612),
        "SAVEGAME_CREATURE_ROAM_CENTER" => Some(16477),
        "SAVEGAME_CREATURE_ROAM_RADIUS" => Some(16476),
        "SAVEGAME_CREATURE_SHOW_AS_ALLY_ON_MAP" => Some(16469),
        "SAVEGAME_CREATURE_STATS" => Some(16209),
        "SAVEGAME_CREATURE_STEALTH" => Some(16454),
        "SAVEGAME_CREATURE_TIMEBEFOREDECAY" => Some(16499),
        "SAVEGAME_CREATURE_TIMESINCEDEATH" => Some(16481),
        "SAVEGAME_CREATURE_TRACKABLE" => Some(16457),
        "SAVEGAME_CURENTQBAR" => Some(16317),
        "SAVEGAME_CURRENT_ACTION_QUEUE" => Some(16740),
        "SAVEGAME_CURRENT_COMMAND" => Some(16720),
        "SAVEGAME_DATAARRAY" => Some(16640),
        "SAVEGAME_DATAARRAY_BOOL" => Some(16643),
        "SAVEGAME_DATAARRAY_FLOAT" => Some(16642),
        "SAVEGAME_DATAARRAY_INT" => Some(16641),
        "SAVEGAME_DATAARRAY_OID" => Some(16644),
        "SAVEGAME_DATAARRAY_QUATERNION" => Some(16647),
        "SAVEGAME_DATAARRAY_STRING" => Some(16645),
        "SAVEGAME_DATAARRAY_VECTOR" => Some(16646),
        "SAVEGAME_DEFAULT_SOUNDSET" => Some(16952),
        "SAVEGAME_EFFECT_ABILITY_ID" => Some(16622),
        "SAVEGAME_EFFECT_ANIMATION" => Some(16619),
        "SAVEGAME_EFFECT_CREATOR" => Some(16621),
        "SAVEGAME_EFFECT_DURATION" => Some(16616),
        "SAVEGAME_EFFECT_DURATION_TYPE" => Some(16615),
        "SAVEGAME_EFFECT_ENGINE_DATA" => Some(16624),
        "SAVEGAME_EFFECT_FLAGS" => Some(16627),
        "SAVEGAME_EFFECT_ID" => Some(16613),
        "SAVEGAME_EFFECT_LIST" => Some(16623),
        "SAVEGAME_EFFECT_PRIORITY" => Some(16620),
        "SAVEGAME_EFFECT_RESOURCE2" => Some(16625),
        "SAVEGAME_EFFECT_STARTINGID" => Some(16626),
        "SAVEGAME_EFFECT_SUBTYPE" => Some(16617),
        "SAVEGAME_EFFECT_TIMEINDEX" => Some(16618),
        "SAVEGAME_EFFECT_TYPE" => Some(16614),
        "SAVEGAME_EQUIPMENT" => Some(16214),
        "SAVEGAME_EQUIPMENTSET" => Some(16215),
        "SAVEGAME_EQUIPMENTSET_OBJECT" => Some(16217),
        "SAVEGAME_EQUIPMENTSET_SLOT" => Some(16216),
        "SAVEGAME_EQUIPMENT_ACTIVESET" => Some(16218),
        "SAVEGAME_EQUIPMENT_ITEMS" => Some(16219),
        "SAVEGAME_EVENT_CALLER_ID" => Some(16633),
        "SAVEGAME_EVENT_DAY" => Some(16631),
        "SAVEGAME_EVENT_ID" => Some(16635),
        "SAVEGAME_EVENT_QUEUE" => Some(16630),
        "SAVEGAME_EVENT_SCRIPT" => Some(16650),
        "SAVEGAME_EVENT_SIMPLE_VALUE" => Some(16651),
        "SAVEGAME_EVENT_TARGET_ID" => Some(16634),
        "SAVEGAME_EVENT_TIME" => Some(16632),
        "SAVEGAME_GAME_STATE" => Some(16005),
        "SAVEGAME_GROUP_HOSTILES" => Some(16452),
        "SAVEGAME_GROUP_ID" => Some(16451),
        "SAVEGAME_GROUP_LIST" => Some(16450),
        "SAVEGAME_HEROIC_PARTY_STATLIST" => Some(16352),
        "SAVEGAME_HEROIC_STATLIST" => Some(16351),
        "SAVEGAME_ISBODYBAG" => Some(16601),
        "SAVEGAME_IS_PLOT_GIVER" => Some(16455),
        "SAVEGAME_ITEMS" => Some(16223),
        "SAVEGAME_ITEM_CURRENT_VFX_PROPERTY_ID" => Some(16233),
        "SAVEGAME_ITEM_CURRENT_VFX_PROPERTY_POWER" => Some(16234),
        "SAVEGAME_ITEM_DAMAGED" => Some(16225),
        "SAVEGAME_ITEM_DROPPABLE" => Some(16224),
        "SAVEGAME_ITEM_INDESTRUCTIBLE" => Some(16229),
        "SAVEGAME_ITEM_INFINITE" => Some(16232),
        "SAVEGAME_ITEM_IRREMOVABLE" => Some(16228),
        "SAVEGAME_ITEM_MATERIALTYPE" => Some(16230),
        "SAVEGAME_ITEM_STEALABLE" => Some(16231),
        "SAVEGAME_JOURNAL" => Some(16504),
        "SAVEGAME_JOURNAL_ACTIVE_LIST" => Some(16505),
        "SAVEGAME_JOURNAL_AREA_TAG" => Some(16512),
        "SAVEGAME_JOURNAL_COMPLETE_LIST" => Some(16506),
        "SAVEGAME_JOURNAL_CONVERSATION_LINE_LIST" => Some(16517),
        "SAVEGAME_JOURNAL_CONVERSATION_LINE_REPLY" => Some(16520),
        "SAVEGAME_JOURNAL_CONVERSATION_LINE_SPEAKER" => Some(16518),
        "SAVEGAME_JOURNAL_CONVERSATION_LINE_TEXT" => Some(16519),
        "SAVEGAME_JOURNAL_CONVERSATION_LIST" => Some(16516),
        "SAVEGAME_JOURNAL_GROUP_LIST" => Some(16525),
        "SAVEGAME_JOURNAL_GROUP_OPEN_IN_COMPLETED" => Some(16528),
        "SAVEGAME_JOURNAL_GROUP_OPEN_IN_CURRENT" => Some(16527),
        "SAVEGAME_JOURNAL_GROUP_PRIORITY" => Some(16529),
        "SAVEGAME_JOURNAL_GROUP_RESREF" => Some(16526),
        "SAVEGAME_JOURNAL_OFFER_ID" => Some(16541),
        "SAVEGAME_JOURNAL_ORPHAN_LIST" => Some(16522),
        "SAVEGAME_JOURNAL_PARENT_PLOT" => Some(16509),
        "SAVEGAME_JOURNAL_PLOT_DESTINATION_GUID_LIST" => Some(16515),
        "SAVEGAME_JOURNAL_PLOT_DESTINATION_LIST" => Some(16513),
        "SAVEGAME_JOURNAL_PLOT_DESTINATION_TAG" => Some(16514),
        "SAVEGAME_JOURNAL_QUEST_COMPLETED" => Some(16523),
        "SAVEGAME_JOURNAL_QUEST_GROUP" => Some(16524),
        "SAVEGAME_JOURNAL_QUEST_UPDATED" => Some(16540),
        "SAVEGAME_JOURNAL_RESREF" => Some(16510),
        "SAVEGAME_JOURNAL_STORY_TEXT" => Some(16511),
        "SAVEGAME_JOURNAL_TEXT" => Some(16508),
        "SAVEGAME_JOURNAL_TITLE" => Some(16507),
        "SAVEGAME_JOURNAL_UNREAD_CODEX_LIST" => Some(16521),
        "SAVEGAME_LOCKQBAR" => Some(16318),
        "SAVEGAME_LOOTABLE_OBJECT_ID" => Some(16602),
        "SAVEGAME_MAX_ITEMS" => Some(16226),
        "SAVEGAME_META_AREANAME" => Some(16800),
        "SAVEGAME_META_BACKGROUND" => Some(16806),
        "SAVEGAME_META_CLASS" => Some(16803),
        "SAVEGAME_META_GENDER" => Some(16804),
        "SAVEGAME_META_LEVEL" => Some(16802),
        "SAVEGAME_META_NAME" => Some(16807),
        "SAVEGAME_META_RACE" => Some(16805),
        "SAVEGAME_META_SAVENAME" => Some(16808),
        "SAVEGAME_META_TIMEPLAYED" => Some(16801),
        "SAVEGAME_MONEY" => Some(16212),
        "SAVEGAME_NONPARTYMEMBERS" => Some(16279),
        "SAVEGAME_OBJECT_ACTIVE" => Some(16201),
        "SAVEGAME_OBJECT_DCDETECTCHECK" => Some(16260),
        "SAVEGAME_OBJECT_DCDISARMCHECK" => Some(16261),
        "SAVEGAME_OBJECT_EVENTSCRIPT" => Some(16221),
        "SAVEGAME_OBJECT_HEALTH" => Some(16251),
        "SAVEGAME_OBJECT_IMMORTAL" => Some(16220),
        "SAVEGAME_OBJECT_IMPORTANCE" => Some(16263),
        "SAVEGAME_OBJECT_INTERACTION_RADIUS" => Some(16262),
        "SAVEGAME_OBJECT_LOOPING_ANIMATION" => Some(16256),
        "SAVEGAME_OBJECT_LOOTABLE_CREATURE_APPEARANCETYPE" => Some(16257),
        "SAVEGAME_OBJECT_MAX_HEALTH" => Some(16252),
        "SAVEGAME_OBJECT_NAME" => Some(16255),
        "SAVEGAME_OBJECT_PICKLOCK" => Some(16258),
        "SAVEGAME_OBJECT_PLOT" => Some(16250),
        "SAVEGAME_OBJECT_RANK" => Some(16253),
        "SAVEGAME_OBJECT_TAG" => Some(16222),
        "SAVEGAME_OBJECT_TRAP_DETECTED" => Some(16259),
        "SAVEGAME_OBJECT_TREASURE_GROUP" => Some(16254),
        "SAVEGAME_PARTYCREATURES" => Some(16207),
        "SAVEGAME_PARTYLIST" => Some(16003),
        "SAVEGAME_PARTYMEMBERS" => Some(16203),
        "SAVEGAME_PARTYMEM_CREATURE" => Some(16205),
        "SAVEGAME_PARTYMEM_TEMPLATE" => Some(16206),
        "SAVEGAME_PARTYPOOLMEMBERS" => Some(16204),
        "SAVEGAME_PARTY_APPROVAL_DESC" => Some(16297),
        "SAVEGAME_PARTY_APPROVAL_ID" => Some(16276),
        "SAVEGAME_PARTY_APPROVAL_LEVEL" => Some(16277),
        "SAVEGAME_PARTY_APPROVAL_LIST" => Some(16275),
        "SAVEGAME_PARTY_AUTO_LEVEL_DEFAULT" => Some(16291),
        "SAVEGAME_PARTY_BACKPACK_SORT" => Some(16299),
        "SAVEGAME_PARTY_HOLD_POSITIONS" => Some(16293),
        "SAVEGAME_PARTY_ITEM_STORAGE_ITEM" => Some(16284),
        "SAVEGAME_PARTY_ITEM_STORAGE_LIST" => Some(16288),
        "SAVEGAME_PARTY_ITEM_STORAGE_OWNER" => Some(16285),
        "SAVEGAME_PARTY_ITEM_STORAGE_SLOT" => Some(16286),
        "SAVEGAME_PARTY_ITEM_STORAGE_WEAPONSET" => Some(16287),
        "SAVEGAME_PARTY_LEADER" => Some(16278),
        "SAVEGAME_PARTY_MEMBER_FOLLOW" => Some(16282),
        "SAVEGAME_PARTY_MEMBER_LOCKED" => Some(16281),
        "SAVEGAME_PARTY_MEMBER_SUBSTATE" => Some(16280),
        "SAVEGAME_PARTY_NEW_ITEM_ID" => Some(16289),
        "SAVEGAME_PARTY_NEW_ITEM_LIST" => Some(16290),
        "SAVEGAME_PARTY_PICKER_GUI_STATUS" => Some(16274),
        "SAVEGAME_PARTY_QUICKBAR_LOCKED" => Some(16292),
        "SAVEGAME_PARTY_RUN_IN_DRIVE_MODE" => Some(16294),
        "SAVEGAME_PARTY_SEEN_LINES" => Some(16503),
        "SAVEGAME_PARTY_TACTICS_ITEM_ABILITIES" => Some(16820),
        "SAVEGAME_PLAYERCHAR" => Some(16002),
        "SAVEGAME_PLAYERCHAR_CHAR" => Some(16208),
        "SAVEGAME_PLAYER_MAP_LEGEND" => Some(16296),
        "SAVEGAME_PLAYER_MAP_ZOOM" => Some(16295),
        "SAVEGAME_PLAYER_MORPH" => Some(16950),
        "SAVEGAME_PLAYER_PORTRAIT_DISTANCE" => Some(16336),
        "SAVEGAME_PLAYER_PORTRAIT_EXPRESSION" => Some(16335),
        "SAVEGAME_PLAYER_PORTRAIT_PITCH" => Some(16332),
        "SAVEGAME_PLAYER_PORTRAIT_POSITIONH" => Some(16337),
        "SAVEGAME_PLAYER_PORTRAIT_POSITIONV" => Some(16338),
        "SAVEGAME_PLAYER_PORTRAIT_TINT" => Some(16334),
        "SAVEGAME_PLAYER_PORTRAIT_YAW" => Some(16333),
        "SAVEGAME_PLAYER_SOUNDSET" => Some(16951),
        "SAVEGAME_PLAYER_TIME_PLAYED" => Some(16298),
        "SAVEGAME_PLOTACTIONS" => Some(16840),
        "SAVEGAME_PLOTACTIONS_CURRENTSET" => Some(16842),
        "SAVEGAME_PLOTACTIONS_ENABLED" => Some(16841),
        "SAVEGAME_PLOTACTIONS_LIST" => Some(16843),
        "SAVEGAME_PLOTACTION_COUNT" => Some(16846),
        "SAVEGAME_PLOTACTION_ID" => Some(16844),
        "SAVEGAME_PLOTACTION_STATE" => Some(16845),
        "SAVEGAME_PLOTACTION_UPDATED" => Some(16847),
        "SAVEGAME_PLOTITEMS" => Some(16211),
        "SAVEGAME_PLOT_FLAGS_1" => Some(16403),
        "SAVEGAME_PLOT_FLAGS_2" => Some(16404),
        "SAVEGAME_PLOT_FLAGS_3" => Some(16405),
        "SAVEGAME_PLOT_FLAGS_4" => Some(16406),
        "SAVEGAME_PLOT_GUID" => Some(16402),
        "SAVEGAME_PLOT_LIST" => Some(16401),
        "SAVEGAME_PLOT_MANAGER" => Some(16400),
        "SAVEGAME_QBAR_EXPANSION_VALUE" => Some(16310),
        "SAVEGAME_QUICKITEMS" => Some(16213),
        "SAVEGAME_QUICKSLOTS" => Some(16308),
        "SAVEGAME_QUICKSLOTS1" => Some(16313),
        "SAVEGAME_QUICKSLOTS2" => Some(16314),
        "SAVEGAME_QUICKSLOTS3" => Some(16315),
        "SAVEGAME_QUICKSLOTS4" => Some(16316),
        "SAVEGAME_QUICKSLOT_ABILITY" => Some(16311),
        "SAVEGAME_QUICKSLOT_ITEMTAG" => Some(16312),
        "SAVEGAME_QUICKSLOT_NUMBER" => Some(16331),
        "SAVEGAME_QUICKSLOT_TEMPLATE" => Some(16319),
        "SAVEGAME_SAVE_VERSION_INTERNAL" => Some(16771),
        "SAVEGAME_SCRIPT_EVENT_CREATOR" => Some(16671),
        "SAVEGAME_SCRIPT_EVENT_DATA" => Some(16673),
        "SAVEGAME_SCRIPT_EVENT_RESOURCE_LIST" => Some(16675),
        "SAVEGAME_SCRIPT_EVENT_SCRIPT_NAME" => Some(16674),
        "SAVEGAME_SCRIPT_EVENT_TARGET" => Some(16672),
        "SAVEGAME_SCRIPT_EVENT_TYPE" => Some(16670),
        "SAVEGAME_SELECTED_CHARACTER" => Some(16270),
        "SAVEGAME_SKILLLIST" => Some(16307),
        "SAVEGAME_SOUND_ACTIVE" => Some(16901),
        "SAVEGAME_SOUND_CONEINSIDE" => Some(16915),
        "SAVEGAME_SOUND_CONEOUTSIDE" => Some(16916),
        "SAVEGAME_SOUND_CONEVOLUME" => Some(16917),
        "SAVEGAME_SOUND_FADEIN" => Some(16912),
        "SAVEGAME_SOUND_FADEOUT" => Some(16913),
        "SAVEGAME_SOUND_MAXDISTANCEMULT" => Some(16914),
        "SAVEGAME_SOUND_NAME" => Some(16902),
        "SAVEGAME_SOUND_OCCLUDABLE" => Some(16919),
        "SAVEGAME_SOUND_PITCH" => Some(16911),
        "SAVEGAME_SOUND_PRIORITY" => Some(16918),
        "SAVEGAME_SOUND_TAG" => Some(16900),
        "SAVEGAME_SOUND_VOLUME" => Some(16910),
        "SAVEGAME_SOUND_WORIENTATION" => Some(16909),
        "SAVEGAME_SOUND_XORIENTATION" => Some(16906),
        "SAVEGAME_SOUND_XPOSITION" => Some(16903),
        "SAVEGAME_SOUND_YORIENTATION" => Some(16907),
        "SAVEGAME_SOUND_YPOSITION" => Some(16904),
        "SAVEGAME_SOUND_ZORIENTATION" => Some(16908),
        "SAVEGAME_SOUND_ZPOSITION" => Some(16905),
        "SAVEGAME_SPELLLIST" => Some(16305),
        "SAVEGAME_STATLIST" => Some(16350),
        "SAVEGAME_STATPROPERTY_BASE" => Some(16300),
        "SAVEGAME_STATPROPERTY_COMREGEN" => Some(16303),
        "SAVEGAME_STATPROPERTY_CURRENT" => Some(16302),
        "SAVEGAME_STATPROPERTY_INDEX" => Some(16353),
        "SAVEGAME_STATPROPERTY_MODIFIER" => Some(16301),
        "SAVEGAME_STATPROPERTY_REGEN" => Some(16304),
        "SAVEGAME_STORE_GOLD" => Some(16152),
        "SAVEGAME_STORE_ITEMLIST" => Some(16156),
        "SAVEGAME_STORE_MARKDOWN" => Some(16150),
        "SAVEGAME_STORE_MARKUP" => Some(16151),
        "SAVEGAME_STORE_MAXBUYPRICE" => Some(16153),
        "SAVEGAME_STORE_WILLNOTBUY" => Some(16154),
        "SAVEGAME_STORE_WILLONLYBUY" => Some(16155),
        "SAVEGAME_STORYSOFAR" => Some(16008),
        "SAVEGAME_STORYSOFAR_AREA" => Some(16976),
        "SAVEGAME_STORYSOFAR_ATTRIBUTE_BASE" => Some(16988),
        "SAVEGAME_STORYSOFAR_ATTRIBUTE_LIST" => Some(16987),
        "SAVEGAME_STORYSOFAR_ATTRIBUTE_MODIFIER" => Some(16989),
        "SAVEGAME_STORYSOFAR_CURRENT_HEATLH" => Some(16979),
        "SAVEGAME_STORYSOFAR_CURRENT_STAMINA" => Some(16981),
        "SAVEGAME_STORYSOFAR_CURRENT_XP" => Some(16983),
        "SAVEGAME_STORYSOFAR_EQUIPMENT_LIST" => Some(16990),
        "SAVEGAME_STORYSOFAR_EQUIPMENT_RESREF" => Some(16992),
        "SAVEGAME_STORYSOFAR_EQUIPMENT_SLOTID" => Some(16991),
        "SAVEGAME_STORYSOFAR_EQUIPMENT_STACKSIZE" => Some(16993),
        "SAVEGAME_STORYSOFAR_EVENTID" => Some(16971),
        "SAVEGAME_STORYSOFAR_EVENTLIST" => Some(16970),
        "SAVEGAME_STORYSOFAR_GAMETIME" => Some(16972),
        "SAVEGAME_STORYSOFAR_ITEM_DATA" => Some(16996),
        "SAVEGAME_STORYSOFAR_ITEM_POWER" => Some(16995),
        "SAVEGAME_STORYSOFAR_ITEM_PROPERTY" => Some(16994),
        "SAVEGAME_STORYSOFAR_LEVEL" => Some(16977),
        "SAVEGAME_STORYSOFAR_LEVELUPLIST" => Some(16975),
        "SAVEGAME_STORYSOFAR_MONEY" => Some(16978),
        "SAVEGAME_STORYSOFAR_SCREENSHOT" => Some(16974),
        "SAVEGAME_STORYSOFAR_SKILL_LIST" => Some(16986),
        "SAVEGAME_STORYSOFAR_SPELL_LIST" => Some(16984),
        "SAVEGAME_STORYSOFAR_TALENT_LIST" => Some(16985),
        "SAVEGAME_STORYSOFAR_TOTAL_HEATLH" => Some(16980),
        "SAVEGAME_STORYSOFAR_TOTAL_STAMINA" => Some(16982),
        "SAVEGAME_STORYSOFAR_UTC" => Some(16973),
        "SAVEGAME_SUBACTION_CORE_INTERRUPTABLE" => Some(16733),
        "SAVEGAME_SUBACTION_CORE_SUBACTION" => Some(16732),
        "SAVEGAME_SUBACTION_DATA" => Some(16738),
        "SAVEGAME_SUBACTION_ID" => Some(16731),
        "SAVEGAME_SUBACTION_LAST_TIME_INDEX" => Some(16735),
        "SAVEGAME_SUBACTION_LENGTH" => Some(16736),
        "SAVEGAME_SUBACTION_LIST" => Some(16730),
        "SAVEGAME_SUBACTION_TIME_INDEX" => Some(16734),
        "SAVEGAME_TACTICENTRY_COMMAND" => Some(16828),
        "SAVEGAME_TACTICENTRY_COMMANDITEMRESREF" => Some(16838),
        "SAVEGAME_TACTICENTRY_COMMANDITEMTAG" => Some(16837),
        "SAVEGAME_TACTICENTRY_COMMANDPARAM" => Some(16829),
        "SAVEGAME_TACTICENTRY_CONDITION" => Some(16827),
        "SAVEGAME_TACTICENTRY_CONDITIONTAG" => Some(16831),
        "SAVEGAME_TACTICENTRY_CONDITION_OBJECT_ID" => Some(16819),
        "SAVEGAME_TACTICENTRY_ENABLED" => Some(16825),
        "SAVEGAME_TACTICENTRY_TARGET" => Some(16826),
        "SAVEGAME_TACTICENTRY_TARGETTAG" => Some(16830),
        "SAVEGAME_TACTICENTRY_TARGET_OBJECT_ID" => Some(16818),
        "SAVEGAME_TACTICS_CUSTOMLIST" => Some(16836),
        "SAVEGAME_TACTICS_DIRTY" => Some(16832),
        "SAVEGAME_TACTICS_ENABLED" => Some(16823),
        "SAVEGAME_TACTICS_HAS_TABLE" => Some(16821),
        "SAVEGAME_TACTICS_LIST" => Some(16824),
        "SAVEGAME_TACTICS_PRESETINDEX" => Some(16834),
        "SAVEGAME_TACTICS_PRESETLIST" => Some(16835),
        "SAVEGAME_TACTICS_PRESETTYPE" => Some(16833),
        "SAVEGAME_TACTICS_TABLE" => Some(16822),
        "SAVEGAME_TALENTLIST" => Some(16306),
        "SAVEGAME_TEAM_ID" => Some(16453),
        "SAVEGAME_VERSION" => Some(16004),
        "SAVEGAME_WAYPOINT_MAPNOTE" => Some(16710),
        "SAVEGAME_WAYPOINT_MAPNOTE_ENABLED" => Some(16711),
        "SAVEGAME_WAYPOINT_MAPNOTE_LOC_TEXT" => Some(16714),
        "SAVEGAME_WAYPOINT_MAPNOTE_TEXT" => Some(16712),
        "SAVEGAME_WAYPOINT_MAPNOTE_TYPE" => Some(16713),
        "SAVEGAME_WORLDDATABASE" => Some(16500),
        "SAVEGAME_WORLDDB_IDGROUP" => Some(16501),
        "SAVEGAME_WORLDDB_LASTID" => Some(16502),
        "SAVEGAME_WORLDMAP" => Some(16780),
        "SAVEGAME_WORLDMAP_GUI_STATUS" => Some(16790),
        "SAVEGAME_WORLDMAP_LAST_PIN_CLICKED" => Some(16791),
        "SAVEGAME_WORLDMAP_MAPLIST" => Some(16783),
        "SAVEGAME_WORLDMAP_MAPPIN_ACTIVATED_PREVIOUSLY" => Some(16792),
        "SAVEGAME_WORLDMAP_MAPPIN_LAST_STATE" => Some(16793),
        "SAVEGAME_WORLDMAP_MAPPIN_NAME" => Some(16798),
        "SAVEGAME_WORLDMAP_MAPPIN_RECENTLY_ACTIVATED" => Some(16789),
        "SAVEGAME_WORLDMAP_MAPPIN_STATE" => Some(16788),
        "SAVEGAME_WORLDMAP_MAPPIN_TAG" => Some(16787),
        "SAVEGAME_WORLDMAP_MAP_PINLIST" => Some(16786),
        "SAVEGAME_WORLDMAP_MAP_PLAYERLOC" => Some(16785),
        "SAVEGAME_WORLDMAP_MAP_TAG" => Some(16784),
        "SAVEGAME_WORLDMAP_MAP_TRAVELPATH_AFTER" => Some(16797),
        "SAVEGAME_WORLDMAP_MAP_TRAVELPATH_BEFORE" => Some(16796),
        "SAVEGAME_WORLDMAP_PRIMARYMAP" => Some(16781),
        "SAVEGAME_WORLDMAP_SECONDARYMAP" => Some(16782),
        "SAVEGAME_WORLDMAP_TRAVELPOINT_POSX" => Some(16794),
        "SAVEGAME_WORLDMAP_TRAVELPOINT_POSY" => Some(16795),
        "SAVEGAME_WORLD_TIMER" => Some(16700),
        "SAVEGAME_WORLD_TIMER_DAY" => Some(16701),
        "SAVEGAME_WORLD_TIMER_TIME" => Some(16702),
        "SAVENAME_SUBACTION_START_TIME" => Some(16737),
        "SAVEPROFILE_ACCOUNT_NAME" => Some(26004),
        "SAVEPROFILE_ACHIEVEMENTLIST" => Some(26006),
        "SAVEPROFILE_ACHIEVEMENT_COUNT" => Some(26010),
        "SAVEPROFILE_ACHIEVEMENT_DATE" => Some(26011),
        "SAVEPROFILE_ACHIEVEMENT_ID" => Some(26007),
        "SAVEPROFILE_ACHIEVEMENT_NEW" => Some(26008),
        "SAVEPROFILE_ACHIEVEMENT_ONLINE" => Some(26009),
        "SAVEPROFILE_ADDIN_LIST" => Some(26100),
        "SAVEPROFILE_ADDIN_TOKEN_LIST" => Some(26111),
        "SAVEPROFILE_BUILD_NUMBER" => Some(26000),
        "SAVEPROFILE_CONTENT_ENABLED" => Some(26104),
        "SAVEPROFILE_CONTENT_NAME" => Some(26102),
        "SAVEPROFILE_CONTENT_SHOWN" => Some(26103),
        "SAVEPROFILE_CONTENT_TOKEN" => Some(26105),
        "SAVEPROFILE_CONTENT_USER" => Some(26106),
        "SAVEPROFILE_FILE_DATA" => Some(26109),
        "SAVEPROFILE_FILE_LIST" => Some(26107),
        "SAVEPROFILE_FILE_NAME" => Some(26108),
        "SAVEPROFILE_FILE_VERSION" => Some(26110),
        "SAVEPROFILE_INITIAL_BUILD_NUMBER" => Some(26001),
        "SAVEPROFILE_LAST_USED_PROFILE" => Some(26002),
        "SAVEPROFILE_LOCAL_ACHIEVEMENT_DATA" => Some(26005),
        "SAVEPROFILE_OFFER_LIST" => Some(26101),
        "SAVEPROFILE_PROFILELIST" => Some(26003),
        "SCRIPTVARTABLE" => Some(17000),
        "SCRIPTVARTABLE_NAME" => Some(17001),
        "SCRIPTVARTABLE_TYPE" => Some(17002),
        "SCRIPTVARTABLE_VALUE" => Some(17003),
        "STAGE_CAMERA_DEPRECATED" => Some(11005),
        "STAGE_CAMERA_FOV" => Some(11003),
        "STAGE_CAMERA_LIST" => Some(11001),
        "STAGE_CAMERA_LOOKING_AT_PRIMARY" => Some(11007),
        "STAGE_CAMERA_LOOKING_AT_SECONDARY" => Some(11008),
        "STAGE_CAMERA_LOOKING_AT_TYPE" => Some(11009),
        "STAGE_CAMERA_LOOKING_FROM" => Some(11006),
        "STAGE_PLACES_IN_SHOT" => Some(11002),
        "STAGE_PLACE_DEFAULT_CAMERA" => Some(11004),
        "STAGE_PLACE_LIST" => Some(11000),
        "TAG" => Some(1),
        "TALK_BUCKET_LIST" => Some(19000),
        "TALK_STRING" => Some(19003),
        "TALK_STRING_ID" => Some(19002),
        "TALK_STRING_LIST" => Some(19001),
        "TEMPLATERESREF" => Some(3),
        "TERRAIN_AREA_BORDER_CELL_WIDTH" => Some(3409),
        "TERRAIN_AREA_CELL_POSITION_X" => Some(3403),
        "TERRAIN_AREA_CELL_POSITION_Y" => Some(3404),
        "TERRAIN_AREA_CELL_POSITION_Z" => Some(3405),
        "TERRAIN_AREA_CELL_SIZE_X" => Some(3406),
        "TERRAIN_AREA_CELL_SIZE_Y" => Some(3407),
        "TERRAIN_AREA_CELL_SIZE_Z" => Some(3408),
        "TERRAIN_AREA_INFORMATION" => Some(7028),
        "TERRAIN_AREA_LIGHTMAP_SIZE" => Some(3411),
        "TERRAIN_AREA_LIGHTMAP_SIZE_VISTA" => Some(3412),
        "TERRAIN_AREA_SUBDIVIDE_BY" => Some(3413),
        "TERRAIN_AREA_VISTA_CELL_WIDTH" => Some(3410),
        "TERRAIN_BASE_COLUMNS" => Some(7002),
        "TERRAIN_BASE_ROWS" => Some(7001),
        "TERRAIN_BLENDPAGE_HEIGHT" => Some(7052),
        "TERRAIN_BLENDPAGE_ID" => Some(7050),
        "TERRAIN_BLENDPAGE_LIST" => Some(7054),
        "TERRAIN_BLENDPAGE_TEXEL_LIST" => Some(7053),
        "TERRAIN_BLENDPAGE_WIDTH" => Some(7051),
        "TERRAIN_BLENDTEXEL_6BYTEWEIGHTLIST" => Some(7070),
        "TERRAIN_BLENDTEXEL_BYTEWEIGHTLIST" => Some(7057),
        "TERRAIN_BLENDTEXEL_ID" => Some(7075),
        "TERRAIN_BLENDTEXEL_WEIGHTLIST" => Some(7049),
        "TERRAIN_BLENDWEIGHT_MATID" => Some(7047),
        "TERRAIN_BLENDWEIGHT_WEIGHT" => Some(7048),
        "TERRAIN_CHUNK" => Some(3154),
        "TERRAIN_CHUNK_BLENDPAGE_SIZE" => Some(3161),
        "TERRAIN_CHUNK_CELL_POSITION_X" => Some(3156),
        "TERRAIN_CHUNK_CELL_POSITION_Y" => Some(3157),
        "TERRAIN_CHUNK_LENGTH" => Some(3158),
        "TERRAIN_CHUNK_LIST" => Some(3155),
        "TERRAIN_CHUNK_SECTOR_ID" => Some(3162),
        "TERRAIN_CHUNK_TEXEL_SIZE" => Some(3160),
        "TERRAIN_CHUNK_WIDTH" => Some(3159),
        "TERRAIN_ELEMENT_ID_SECTOR" => Some(7025),
        "TERRAIN_ELEMENT_ID_VALUE" => Some(7024),
        "TERRAIN_EXPORT_AREA" => Some(3400),
        "TERRAIN_EXPORT_AREA_LIST" => Some(3401),
        "TERRAIN_LENGTH_UNITS" => Some(7003),
        "TERRAIN_MAPEDGE_ID" => Some(7040),
        "TERRAIN_MAPEDGE_LIST" => Some(7042),
        "TERRAIN_MAPEDGE_START_VERTEX" => Some(7041),
        "TERRAIN_MAPFACE_BLENDPAGE_ID" => Some(7046),
        "TERRAIN_MAPFACE_ID" => Some(7043),
        "TERRAIN_MAPFACE_LAYER" => Some(7044),
        "TERRAIN_MAPFACE_LIST" => Some(7045),
        "TERRAIN_MAPVERTEX_ID" => Some(7037),
        "TERRAIN_MAPVERTEX_LIST" => Some(7039),
        "TERRAIN_MAPVERTEX_UVW" => Some(7038),
        "TERRAIN_MATERIAL" => Some(7060),
        "TERRAIN_MATERIAL_DIFFUSE_NAME" => Some(7064),
        "TERRAIN_MATERIAL_HEIGHTMAP_NAME" => Some(7067),
        "TERRAIN_MATERIAL_ID" => Some(7061),
        "TERRAIN_MATERIAL_LIST" => Some(7027),
        "TERRAIN_MATERIAL_NAME" => Some(7062),
        "TERRAIN_MATERIAL_NORMAL_NAME" => Some(7065),
        "TERRAIN_MATERIAL_RELIEF_SCALE" => Some(7069),
        "TERRAIN_MATERIAL_SCALE" => Some(7063),
        "TERRAIN_MATERIAL_SPECUALAR_NAME" => Some(7066),
        "TERRAIN_MATERIAL_SPECULAR_COLOR" => Some(7077),
        "TERRAIN_MATERIAL_VALUE" => Some(7026),
        "TERRAIN_MESH" => Some(7055),
        "TERRAIN_MESHEDGE_ID" => Some(7012),
        "TERRAIN_MESHEDGE_LIST" => Some(7016),
        "TERRAIN_MESHEDGE_START_VERTEX" => Some(7013),
        "TERRAIN_MESHEDGE_SUBEDGE_A" => Some(7073),
        "TERRAIN_MESHEDGE_SUBEDGE_B" => Some(7074),
        "TERRAIN_MESHEDGE_SUBEDGE_LIST" => Some(7015),
        "TERRAIN_MESHFACE_ID" => Some(7010),
        "TERRAIN_MESHFACE_LIST" => Some(7011),
        "TERRAIN_MESHVERTEX_CONSTRAINT_A" => Some(7071),
        "TERRAIN_MESHVERTEX_CONSTRAINT_B" => Some(7072),
        "TERRAIN_MESHVERTEX_CONSTRAINT_ID" => Some(7022),
        "TERRAIN_MESHVERTEX_CONSTRAINT_LIST" => Some(7021),
        "TERRAIN_MESHVERTEX_ID" => Some(7018),
        "TERRAIN_MESHVERTEX_LEVEL" => Some(7020),
        "TERRAIN_MESHVERTEX_LIST" => Some(7023),
        "TERRAIN_MESHVERTEX_POSITION" => Some(7019),
        "TERRAIN_MESH_NAME" => Some(7058),
        "TERRAIN_PALETTE" => Some(7056),
        "TERRAIN_PALETTE_NAME" => Some(7059),
        "TERRAIN_PALETTE_PARALLAX_GLOBAL" => Some(7068),
        "TERRAIN_SECTOR_COLUMNS" => Some(7006),
        "TERRAIN_SECTOR_ID" => Some(7008),
        "TERRAIN_SECTOR_LIST" => Some(7009),
        "TERRAIN_SECTOR_ROWS" => Some(7005),
        "TERRAIN_SOUND_DATA" => Some(7076),
        "TERRAIN_SUBEDGE_ID" => Some(7017),
        "TERRAIN_TESSELLATION" => Some(7007),
        "TERRAIN_VERSION" => Some(7000),
        "TERRAIN_VERTEX_U" => Some(7029),
        "TERRAIN_VERTEX_V" => Some(7030),
        "TERRAIN_WIDTH_UNITS" => Some(7004),
        "TEXT" => Some(22),
        "TINT_MASK_DIFFUSE_A" => Some(14006),
        "TINT_MASK_DIFFUSE_B" => Some(14002),
        "TINT_MASK_DIFFUSE_G" => Some(14001),
        "TINT_MASK_DIFFUSE_OPACITY" => Some(14008),
        "TINT_MASK_DIFFUSE_R" => Some(14000),
        "TINT_MASK_SPECULAR_A" => Some(14007),
        "TINT_MASK_SPECULAR_B" => Some(14005),
        "TINT_MASK_SPECULAR_G" => Some(14004),
        "TINT_MASK_SPECULAR_OPACITY" => Some(14009),
        "TINT_MASK_SPECULAR_R" => Some(14003),
        "TS_PROPERTY" => Some(900),
        "TS_PROPERTY_ATOM" => Some(902),
        "TS_PROPERTY_CHILDREN" => Some(904),
        "TS_PROPERTY_NAME" => Some(901),
        "TS_PROPERTY_VALUE" => Some(903),
        "TS_PROPERTY_VARTYPE" => Some(905),
        "UINT16_LIST" => Some(8),
        "UINT32_LIST" => Some(10),
        "UINT64_LIST" => Some(12),
        "UINT8_LIST" => Some(6),
        "VECTOR3F_LIST" => Some(16),
        "VECTOR4F_LIST" => Some(17),
        "VFX_AGEMAP_COLOR_A" => Some(21143),
        "VFX_AGEMAP_COLOR_B" => Some(21142),
        "VFX_AGEMAP_COLOR_G" => Some(21141),
        "VFX_AGEMAP_COLOR_R" => Some(21140),
        "VFX_AGEMAP_ROTATIONAL_SPEED_MULTIPLIER" => Some(21146),
        "VFX_AGEMAP_SCALE_X" => Some(21144),
        "VFX_AGEMAP_SCALE_Y" => Some(21145),
        "VFX_CESSATION_LENGTH" => Some(21132),
        "VFX_CHILD_LIST" => Some(21000),
        "VFX_CREATURE_NAME" => Some(21110),
        "VFX_CREATURE_URI" => Some(21111),
        "VFX_CRUSTNODE_CRUSTHOOKID" => Some(21062),
        "VFX_CRUSTNODE_NAME" => Some(21060),
        "VFX_CRUSTNODE_REALNAME" => Some(21061),
        "VFX_CUSTOM_LENGTH" => Some(21133),
        "VFX_CUSTOM_NAME" => Some(21134),
        "VFX_DUMMY_NAME" => Some(21070),
        "VFX_DURATION_LENGTH" => Some(21131),
        "VFX_EMITTER_ACCELERATION" => Some(21023),
        "VFX_EMITTER_AGENT" => Some(21064),
        "VFX_EMITTER_ALPHAMULTIPLIER" => Some(21054),
        "VFX_EMITTER_AXIS_ACCELERATION_SPACE" => Some(21197),
        "VFX_EMITTER_BIRTHRATE" => Some(21018),
        "VFX_EMITTER_BIRTHRATEINPARTICLESPERMETER" => Some(21020),
        "VFX_EMITTER_BIRTHRATERANGE" => Some(21019),
        "VFX_EMITTER_BOUNCE_VALUE" => Some(21164),
        "VFX_EMITTER_CHUNKY_MODEL_NAME" => Some(21058),
        "VFX_EMITTER_COLLISION_TYPE" => Some(21163),
        "VFX_EMITTER_COLORMULTIPLIER" => Some(21055),
        "VFX_EMITTER_COLORMULTIPLIER_B" => Some(21172),
        "VFX_EMITTER_COLORMULTIPLIER_G" => Some(21171),
        "VFX_EMITTER_COLORMULTIPLIER_R" => Some(21170),
        "VFX_EMITTER_ENABLEPARTICLECOLLISIONS" => Some(21037),
        "VFX_EMITTER_FLIPBOOK_COLUMNS" => Some(21052),
        "VFX_EMITTER_FLIPBOOK_FRAMES_PER_SECOND" => Some(21050),
        "VFX_EMITTER_FLIPBOOK_RANDOM_START_FRAME" => Some(21053),
        "VFX_EMITTER_FLIPBOOK_ROWS" => Some(21051),
        "VFX_EMITTER_FLIPBOOK_TYPE" => Some(21049),
        "VFX_EMITTER_GRAVITYMULTIPLIER" => Some(21024),
        "VFX_EMITTER_GROUP_NAME" => Some(21210),
        "VFX_EMITTER_INFINITELIFE" => Some(21057),
        "VFX_EMITTER_INHERITVELOCITYINSTEADOFPOSITION" => Some(21035),
        "VFX_EMITTER_INITIALROTATION" => Some(21059),
        "VFX_EMITTER_INITIALROTATIONRANGE" => Some(21002),
        "VFX_EMITTER_INITIALROTATIONSPEED" => Some(21030),
        "VFX_EMITTER_INITIALROTATIONSPEEDRANGE" => Some(21031),
        "VFX_EMITTER_INITIALSPEED" => Some(21021),
        "VFX_EMITTER_INITIALSPEEDRANGE" => Some(21022),
        "VFX_EMITTER_KILLPARTICLEWHENTARGETHIT" => Some(21047),
        "VFX_EMITTER_LIFE" => Some(21025),
        "VFX_EMITTER_LIFERANGE" => Some(21026),
        "VFX_EMITTER_LINKPARTICLESTOGETHER" => Some(21015),
        "VFX_EMITTER_MATERIALLIBRARY" => Some(21016),
        "VFX_EMITTER_MATERIALOBJECT" => Some(21017),
        "VFX_EMITTER_MESH_PARTICLE_ROLL_AXIS" => Some(21005),
        "VFX_EMITTER_MESH_PARTICLE_UP_AXIS" => Some(21008),
        "VFX_EMITTER_MOVEMENTSPREADUPDATEDELAY" => Some(21042),
        "VFX_EMITTER_MOVEMENTSPREADX" => Some(21040),
        "VFX_EMITTER_MOVEMENTSPREADY" => Some(21041),
        "VFX_EMITTER_NAME" => Some(21011),
        "VFX_EMITTER_ORIENTATIONBEHAVIOUR" => Some(21013),
        "VFX_EMITTER_PARTICLEINHERITANCE" => Some(21034),
        "VFX_EMITTER_PARTICLESAFFECTEDBYWIND" => Some(21036),
        "VFX_EMITTER_PARTICLESFOLLOWPATH" => Some(21048),
        "VFX_EMITTER_PHYSICSEMITTER" => Some(21039),
        "VFX_EMITTER_PHYSICSOBJECTSPAWN" => Some(21038),
        "VFX_EMITTER_RANDOMINITIALROTATION" => Some(21033),
        "VFX_EMITTER_ROTATIONALACCELERATION" => Some(21032),
        "VFX_EMITTER_SCALEMULTIPLIER" => Some(21056),
        "VFX_EMITTER_SCALERANGE" => Some(21027),
        "VFX_EMITTER_SPAWNDIRECTIONTRACKSTARGET" => Some(21046),
        "VFX_EMITTER_SPLAT_ALPHAMULTIPLIER" => Some(21181),
        "VFX_EMITTER_SPLAT_COLORMULTIPLIER_B" => Some(21184),
        "VFX_EMITTER_SPLAT_COLORMULTIPLIER_G" => Some(21183),
        "VFX_EMITTER_SPLAT_COLORMULTIPLIER_R" => Some(21182),
        "VFX_EMITTER_SPREADX" => Some(21028),
        "VFX_EMITTER_SPREADY" => Some(21029),
        "VFX_EMITTER_TARGETATTRACTION" => Some(21044),
        "VFX_EMITTER_TARGETNAME" => Some(21043),
        "VFX_EMITTER_TARGETRADIUS" => Some(21045),
        "VFX_EMITTER_TYPE" => Some(21012),
        "VFX_EMITTER_UPDATEONLYWHENVISIBLE" => Some(21014),
        "VFX_EMITTER_UVDISTRIBUTIONSIZE" => Some(21198),
        "VFX_EMITTER_VOLUME_SPAWN_ARBITRARY_VOLUME_NAME" => Some(21162),
        "VFX_EMITTER_VOLUME_SPAWN_INVERT_NORMALS" => Some(21166),
        "VFX_EMITTER_VOLUME_SPAWN_PRIMITIVE_BOX_MAX" => Some(21187),
        "VFX_EMITTER_VOLUME_SPAWN_PRIMITIVE_BOX_MIN" => Some(21186),
        "VFX_EMITTER_VOLUME_SPAWN_PRIMITIVE_CYLINDER_AXIS" => Some(21191),
        "VFX_EMITTER_VOLUME_SPAWN_PRIMITIVE_CYLINDER_H" => Some(21190),
        "VFX_EMITTER_VOLUME_SPAWN_PRIMITIVE_CYLINDER_R" => Some(21189),
        "VFX_EMITTER_VOLUME_SPAWN_PRIMITIVE_MESH_TYPE" => Some(21185),
        "VFX_EMITTER_VOLUME_SPAWN_PRIMITIVE_SPHERE_R" => Some(21188),
        "VFX_EMITTER_VOLUME_SPAWN_SELECTED_PART_NAME" => Some(21161),
        "VFX_EMITTER_VOLUME_SPAWN_TYPE" => Some(21160),
        "VFX_EMITTER_VOLUME_SPAWN_USE_VOLUME_NORMAL" => Some(21192),
        "VFX_EMITTER_VOLUME_SPAWN_WITHIN_VOLUME" => Some(21165),
        "VFX_EMITTER_WORLD_AXIS_ACCELERATION_X" => Some(21193),
        "VFX_EMITTER_WORLD_AXIS_ACCELERATION_Y" => Some(21194),
        "VFX_EMITTER_WORLD_AXIS_ACCELERATION_Z" => Some(21195),
        "VFX_EVENT" => Some(21150),
        "VFX_EVENT_ID" => Some(21153),
        "VFX_EVENT_TARGETSYSTEM" => Some(21154),
        "VFX_EVENT_TIME" => Some(21151),
        "VFX_EVENT_TYPE" => Some(21152),
        "VFX_FILE_OBJECT_VERSION" => Some(21180),
        "VFX_GEOMETRY_FILE_NAME" => Some(21063),
        "VFX_GEOMETRY_NAME" => Some(21080),
        "VFX_GEOMETRY_SCALE" => Some(21081),
        "VFX_IMPACT_LENGTH" => Some(21130),
        "VFX_KEYFRAME" => Some(21009),
        "VFX_MODEL_ANIMATIONNAME" => Some(21102),
        "VFX_MODEL_NAME" => Some(21100),
        "VFX_MODEL_RESOURCETYPE" => Some(21101),
        "VFX_OBJECT_ID" => Some(21001),
        "VFX_OBJECT_VISIBLE" => Some(21007),
        "VFX_RANGE" => Some(21196),
        "VFX_RELATIVE_ORIENTATION_X" => Some(21123),
        "VFX_RELATIVE_ORIENTATION_Y" => Some(21124),
        "VFX_RELATIVE_ORIENTATION_Z" => Some(21125),
        "VFX_RELATIVE_POSITION_X" => Some(21120),
        "VFX_RELATIVE_POSITION_Y" => Some(21121),
        "VFX_RELATIVE_POSITION_Z" => Some(21122),
        "VFX_REMOTE_MATERIAL_ALPHA" => Some(21226),
        "VFX_REMOTE_MATERIAL_DECAL_NAME" => Some(21227),
        "VFX_REMOTE_MATERIAL_FRESNEL_FALLOFF" => Some(21224),
        "VFX_REMOTE_MATERIAL_INVERT_FRESNEL" => Some(21225),
        "VFX_REMOTE_MATERIAL_TINT_A" => Some(21223),
        "VFX_REMOTE_MATERIAL_TINT_B" => Some(21222),
        "VFX_REMOTE_MATERIAL_TINT_G" => Some(21221),
        "VFX_REMOTE_MATERIAL_TINT_R" => Some(21220),
        "VFX_ROOT" => Some(21004),
        "VFX_SPLAT_AGEMAP_COLOR_A" => Some(21176),
        "VFX_SPLAT_AGEMAP_COLOR_B" => Some(21175),
        "VFX_SPLAT_AGEMAP_COLOR_G" => Some(21174),
        "VFX_SPLAT_AGEMAP_COLOR_R" => Some(21173),
        "VFX_SPLAT_AGEMAP_SCALE_X" => Some(21177),
        "VFX_SPLAT_AGEMAP_SCALE_Y" => Some(21178),
        "VFX_TARGET_NAME" => Some(21090),
        "VFX_TYPE" => Some(21006),
        "VFX_USE_VARIATION_TINT" => Some(21065),
        "VFX_VALUE" => Some(21010),
        "WATER_ID" => Some(7902),
        "WATER_INFORMATION" => Some(7900),
        "WATER_VERSION" => Some(7901),
        "WATER_VERTEX_COLOR" => Some(7907),
        "WATER_VERTEX_INDEX_LIST" => Some(7908),
        "WATER_VERTEX_LIST" => Some(7903),
        "WATER_VERTEX_NORMAL" => Some(7905),
        "WATER_VERTEX_POSITION" => Some(7904),
        "WATER_VERTEX_UVW" => Some(7906),
        "WND_CLOTH_GUST_AXIS_RATIO" => Some(22041),
        "WND_CLOTH_GUST_DIR_CHANGE" => Some(22040),
        "WND_CLOTH_GUST_DURATION_MAX" => Some(22037),
        "WND_CLOTH_GUST_DURATION_MIN" => Some(22036),
        "WND_CLOTH_GUST_INTERVAL_MAX" => Some(22039),
        "WND_CLOTH_GUST_INTERVAL_MIN" => Some(22038),
        "WND_CLOTH_GUST_STRENGTH_MAX" => Some(22035),
        "WND_CLOTH_GUST_STRENGTH_MIN" => Some(22034),
        "WND_CLOTH_RESPONSE" => Some(22031),
        "WND_CLOTH_RESPONSE_LMT" => Some(22032),
        "WND_CLOTH_STRENGTH" => Some(22033),
        "WND_DIRECTION" => Some(22004),
        "WND_GUST_FREQUENCY" => Some(22014),
        "WND_GUST_MAX_DURATION" => Some(22013),
        "WND_GUST_MAX_STRENGTH" => Some(22011),
        "WND_GUST_MIN_DURATION" => Some(22012),
        "WND_GUST_MIN_STRENGTH" => Some(22010),
        "WND_RADIUS" => Some(22002),
        "WND_RADIUS_FALLOFF" => Some(22005),
        "WND_RESREF" => Some(22001),
        "WND_ROOT" => Some(22000),
        "WND_STRENGTH" => Some(22003),
        "WND_TREE_BRANCH_EXPONENT" => Some(22025),
        "WND_TREE_BRANCH_OSCILLATION_X" => Some(22027),
        "WND_TREE_BRANCH_OSCILLATION_Y" => Some(22028),
        "WND_TREE_LEAF_EXPONENT" => Some(22026),
        "WND_TREE_LEAF_ROCKING" => Some(22029),
        "WND_TREE_LEAF_RUSTLING" => Some(22030),
        "WND_TREE_MAX_BEND_ANGLE" => Some(22024),
        "WND_TREE_NUM_LEAF_ANGLES" => Some(22021),
        "WND_TREE_NUM_WIND_MATRICES" => Some(22020),
        "WND_TREE_RESPONSE" => Some(22022),
        "WND_TREE_RESPONSE_LIMIT" => Some(22023),
        _ => None,
    }
}
