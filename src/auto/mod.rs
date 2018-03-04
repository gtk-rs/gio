// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

mod action;
pub use self::action::Action;
pub use self::action::ActionExt;

mod action_group;
pub use self::action_group::ActionGroup;
pub use self::action_group::ActionGroupExt;

mod action_map;
pub use self::action_map::ActionMap;
pub use self::action_map::ActionMapExt;

mod app_info;
pub use self::app_info::AppInfo;
pub use self::app_info::AppInfoExt;

mod app_launch_context;
pub use self::app_launch_context::AppLaunchContext;
pub use self::app_launch_context::AppLaunchContextExt;

mod application;
pub use self::application::Application;
pub use self::application::ApplicationExt;

mod buffered_input_stream;
pub use self::buffered_input_stream::BufferedInputStream;
pub use self::buffered_input_stream::BufferedInputStreamExt;

mod buffered_output_stream;
pub use self::buffered_output_stream::BufferedOutputStream;
pub use self::buffered_output_stream::BufferedOutputStreamExt;

mod cancellable;
pub use self::cancellable::Cancellable;
pub use self::cancellable::CancellableExt;

mod data_input_stream;
pub use self::data_input_stream::DataInputStream;
pub use self::data_input_stream::DataInputStreamExt;

mod data_output_stream;
pub use self::data_output_stream::DataOutputStream;
pub use self::data_output_stream::DataOutputStreamExt;

mod file;
pub use self::file::File;
pub use self::file::FileExt;

mod file_i_o_stream;
pub use self::file_i_o_stream::FileIOStream;
pub use self::file_i_o_stream::FileIOStreamExt;

mod file_info;
pub use self::file_info::FileInfo;
pub use self::file_info::FileInfoExt;

mod file_input_stream;
pub use self::file_input_stream::FileInputStream;
pub use self::file_input_stream::FileInputStreamExt;

mod file_output_stream;
pub use self::file_output_stream::FileOutputStream;
pub use self::file_output_stream::FileOutputStreamExt;

mod filter_input_stream;
pub use self::filter_input_stream::FilterInputStream;
pub use self::filter_input_stream::FilterInputStreamExt;

mod filter_output_stream;
pub use self::filter_output_stream::FilterOutputStream;
pub use self::filter_output_stream::FilterOutputStreamExt;

mod i_o_stream;
pub use self::i_o_stream::IOStream;
pub use self::i_o_stream::IOStreamExt;

mod icon;
pub use self::icon::Icon;
pub use self::icon::IconExt;

mod inet_address;
pub use self::inet_address::InetAddress;
pub use self::inet_address::InetAddressExt;

mod inet_socket_address;
pub use self::inet_socket_address::InetSocketAddress;
pub use self::inet_socket_address::InetSocketAddressExt;

mod input_stream;
pub use self::input_stream::InputStream;
pub use self::input_stream::InputStreamExt;

mod memory_input_stream;
pub use self::memory_input_stream::MemoryInputStream;
pub use self::memory_input_stream::MemoryInputStreamExt;

mod memory_output_stream;
pub use self::memory_output_stream::MemoryOutputStream;
pub use self::memory_output_stream::MemoryOutputStreamExt;

mod menu;
pub use self::menu::Menu;
pub use self::menu::MenuExt;

mod menu_attribute_iter;
pub use self::menu_attribute_iter::MenuAttributeIter;
pub use self::menu_attribute_iter::MenuAttributeIterExt;

mod menu_item;
pub use self::menu_item::MenuItem;
pub use self::menu_item::MenuItemExt;

mod menu_link_iter;
pub use self::menu_link_iter::MenuLinkIter;
pub use self::menu_link_iter::MenuLinkIterExt;

mod menu_model;
pub use self::menu_model::MenuModel;
pub use self::menu_model::MenuModelExt;

mod mount_operation;
pub use self::mount_operation::MountOperation;
pub use self::mount_operation::MountOperationExt;

mod network_address;
pub use self::network_address::NetworkAddress;
pub use self::network_address::NetworkAddressExt;

mod network_service;
pub use self::network_service::NetworkService;
pub use self::network_service::NetworkServiceExt;

#[cfg(any(feature = "v2_40", feature = "dox"))]
mod notification;
#[cfg(any(feature = "v2_40", feature = "dox"))]
pub use self::notification::Notification;
#[cfg(any(feature = "v2_40", feature = "dox"))]
pub use self::notification::NotificationExt;

mod output_stream;
pub use self::output_stream::OutputStream;
pub use self::output_stream::OutputStreamExt;

mod permission;
pub use self::permission::Permission;
pub use self::permission::PermissionExt;

mod resolver;
pub use self::resolver::Resolver;
pub use self::resolver::ResolverExt;

mod seekable;
pub use self::seekable::Seekable;
pub use self::seekable::SeekableExt;

mod settings;
pub use self::settings::Settings;
pub use self::settings::SettingsExt;

mod simple_action;
pub use self::simple_action::SimpleAction;
pub use self::simple_action::SimpleActionExt;

mod simple_action_group;
pub use self::simple_action_group::SimpleActionGroup;
pub use self::simple_action_group::SimpleActionGroupExt;

#[cfg(any(feature = "v2_44", feature = "dox"))]
mod simple_i_o_stream;
#[cfg(any(feature = "v2_44", feature = "dox"))]
pub use self::simple_i_o_stream::SimpleIOStream;

mod simple_permission;
pub use self::simple_permission::SimplePermission;

mod socket;
pub use self::socket::Socket;
pub use self::socket::SocketExt;

mod socket_address;
pub use self::socket_address::SocketAddress;
pub use self::socket_address::SocketAddressExt;

mod socket_address_enumerator;
pub use self::socket_address_enumerator::SocketAddressEnumerator;
pub use self::socket_address_enumerator::SocketAddressEnumeratorExt;

mod socket_client;
pub use self::socket_client::SocketClient;
pub use self::socket_client::SocketClientExt;

mod socket_connectable;
pub use self::socket_connectable::SocketConnectable;
pub use self::socket_connectable::SocketConnectableExt;

mod socket_connection;
pub use self::socket_connection::SocketConnection;
pub use self::socket_connection::SocketConnectionExt;

mod socket_listener;
pub use self::socket_listener::SocketListener;
pub use self::socket_listener::SocketListenerExt;

mod socket_service;
pub use self::socket_service::SocketService;
pub use self::socket_service::SocketServiceExt;

mod tcp_connection;
pub use self::tcp_connection::TcpConnection;
pub use self::tcp_connection::TcpConnectionExt;

mod themed_icon;
pub use self::themed_icon::ThemedIcon;
pub use self::themed_icon::ThemedIconExt;

mod threaded_socket_service;
pub use self::threaded_socket_service::ThreadedSocketService;
pub use self::threaded_socket_service::ThreadedSocketServiceExt;

mod tls_certificate;
pub use self::tls_certificate::TlsCertificate;
pub use self::tls_certificate::TlsCertificateExt;

mod tls_client_connection;
pub use self::tls_client_connection::TlsClientConnection;
pub use self::tls_client_connection::TlsClientConnectionExt;

mod tls_connection;
pub use self::tls_connection::TlsConnection;
pub use self::tls_connection::TlsConnectionExt;

mod tls_database;
pub use self::tls_database::TlsDatabase;
pub use self::tls_database::TlsDatabaseExt;

mod tls_file_database;
pub use self::tls_file_database::TlsFileDatabase;
pub use self::tls_file_database::TlsFileDatabaseExt;

mod tls_interaction;
pub use self::tls_interaction::TlsInteraction;
pub use self::tls_interaction::TlsInteractionExt;

mod tls_password;
pub use self::tls_password::TlsPassword;
pub use self::tls_password::TlsPasswordExt;

mod tls_server_connection;
pub use self::tls_server_connection::TlsServerConnection;
pub use self::tls_server_connection::TlsServerConnectionExt;

mod resource;
pub use self::resource::Resource;

mod settings_schema;
pub use self::settings_schema::SettingsSchema;

#[cfg(any(feature = "v2_40", feature = "dox"))]
mod settings_schema_key;
#[cfg(any(feature = "v2_40", feature = "dox"))]
pub use self::settings_schema_key::SettingsSchemaKey;

mod srv_target;
pub use self::srv_target::SrvTarget;

mod enums;
pub use self::enums::DataStreamByteOrder;
pub use self::enums::DataStreamNewlineType;
pub use self::enums::FileType;
pub use self::enums::IOErrorEnum;
pub use self::enums::MountOperationResult;
#[cfg(any(feature = "v2_42", feature = "dox"))]
pub use self::enums::NotificationPriority;
pub use self::enums::PasswordSave;
#[cfg(any(feature = "v2_34", feature = "dox"))]
pub use self::enums::ResolverRecordType;
pub use self::enums::ResourceError;
pub use self::enums::SocketClientEvent;
pub use self::enums::SocketFamily;
#[cfg(any(feature = "v2_46", feature = "dox"))]
pub use self::enums::SocketListenerEvent;
pub use self::enums::SocketProtocol;
pub use self::enums::SocketType;
pub use self::enums::TlsAuthenticationMode;
#[cfg(any(feature = "v2_40", feature = "dox"))]
pub use self::enums::TlsCertificateRequestFlags;
pub use self::enums::TlsDatabaseLookupFlags;
pub use self::enums::TlsInteractionResult;
pub use self::enums::TlsRehandshakeMode;

mod flags;
pub use self::flags::AppInfoCreateFlags;
pub use self::flags::ApplicationFlags;
pub use self::flags::AskPasswordFlags;
pub use self::flags::FileCreateFlags;
pub use self::flags::FileQueryInfoFlags;
pub use self::flags::IOStreamSpliceFlags;
pub use self::flags::OutputStreamSpliceFlags;
pub use self::flags::ResourceLookupFlags;
pub use self::flags::SettingsBindFlags;
pub use self::flags::TlsCertificateFlags;
pub use self::flags::TlsDatabaseVerifyFlags;
pub use self::flags::TlsPasswordFlags;

pub mod functions;

mod constants;
pub use self::constants::DESKTOP_APP_INFO_LOOKUP_EXTENSION_POINT_NAME;
pub use self::constants::FILE_ATTRIBUTE_ACCESS_CAN_DELETE;
pub use self::constants::FILE_ATTRIBUTE_ACCESS_CAN_EXECUTE;
pub use self::constants::FILE_ATTRIBUTE_ACCESS_CAN_READ;
pub use self::constants::FILE_ATTRIBUTE_ACCESS_CAN_RENAME;
pub use self::constants::FILE_ATTRIBUTE_ACCESS_CAN_TRASH;
pub use self::constants::FILE_ATTRIBUTE_ACCESS_CAN_WRITE;
pub use self::constants::FILE_ATTRIBUTE_DOS_IS_ARCHIVE;
pub use self::constants::FILE_ATTRIBUTE_DOS_IS_SYSTEM;
pub use self::constants::FILE_ATTRIBUTE_ETAG_VALUE;
pub use self::constants::FILE_ATTRIBUTE_FILESYSTEM_FREE;
pub use self::constants::FILE_ATTRIBUTE_FILESYSTEM_READONLY;
pub use self::constants::FILE_ATTRIBUTE_FILESYSTEM_REMOTE;
pub use self::constants::FILE_ATTRIBUTE_FILESYSTEM_SIZE;
pub use self::constants::FILE_ATTRIBUTE_FILESYSTEM_TYPE;
pub use self::constants::FILE_ATTRIBUTE_FILESYSTEM_USED;
pub use self::constants::FILE_ATTRIBUTE_FILESYSTEM_USE_PREVIEW;
pub use self::constants::FILE_ATTRIBUTE_GVFS_BACKEND;
pub use self::constants::FILE_ATTRIBUTE_ID_FILE;
pub use self::constants::FILE_ATTRIBUTE_ID_FILESYSTEM;
pub use self::constants::FILE_ATTRIBUTE_MOUNTABLE_CAN_EJECT;
pub use self::constants::FILE_ATTRIBUTE_MOUNTABLE_CAN_MOUNT;
pub use self::constants::FILE_ATTRIBUTE_MOUNTABLE_CAN_POLL;
pub use self::constants::FILE_ATTRIBUTE_MOUNTABLE_CAN_START;
pub use self::constants::FILE_ATTRIBUTE_MOUNTABLE_CAN_START_DEGRADED;
pub use self::constants::FILE_ATTRIBUTE_MOUNTABLE_CAN_STOP;
pub use self::constants::FILE_ATTRIBUTE_MOUNTABLE_CAN_UNMOUNT;
pub use self::constants::FILE_ATTRIBUTE_MOUNTABLE_HAL_UDI;
pub use self::constants::FILE_ATTRIBUTE_MOUNTABLE_IS_MEDIA_CHECK_AUTOMATIC;
pub use self::constants::FILE_ATTRIBUTE_MOUNTABLE_START_STOP_TYPE;
pub use self::constants::FILE_ATTRIBUTE_MOUNTABLE_UNIX_DEVICE;
pub use self::constants::FILE_ATTRIBUTE_MOUNTABLE_UNIX_DEVICE_FILE;
pub use self::constants::FILE_ATTRIBUTE_OWNER_GROUP;
pub use self::constants::FILE_ATTRIBUTE_OWNER_USER;
pub use self::constants::FILE_ATTRIBUTE_OWNER_USER_REAL;
pub use self::constants::FILE_ATTRIBUTE_PREVIEW_ICON;
#[cfg(any(feature = "v2_52", feature = "dox"))]
pub use self::constants::FILE_ATTRIBUTE_RECENT_MODIFIED;
pub use self::constants::FILE_ATTRIBUTE_SELINUX_CONTEXT;
pub use self::constants::FILE_ATTRIBUTE_STANDARD_ALLOCATED_SIZE;
pub use self::constants::FILE_ATTRIBUTE_STANDARD_CONTENT_TYPE;
pub use self::constants::FILE_ATTRIBUTE_STANDARD_COPY_NAME;
pub use self::constants::FILE_ATTRIBUTE_STANDARD_DESCRIPTION;
pub use self::constants::FILE_ATTRIBUTE_STANDARD_DISPLAY_NAME;
pub use self::constants::FILE_ATTRIBUTE_STANDARD_EDIT_NAME;
pub use self::constants::FILE_ATTRIBUTE_STANDARD_FAST_CONTENT_TYPE;
pub use self::constants::FILE_ATTRIBUTE_STANDARD_ICON;
pub use self::constants::FILE_ATTRIBUTE_STANDARD_IS_BACKUP;
pub use self::constants::FILE_ATTRIBUTE_STANDARD_IS_HIDDEN;
pub use self::constants::FILE_ATTRIBUTE_STANDARD_IS_SYMLINK;
pub use self::constants::FILE_ATTRIBUTE_STANDARD_IS_VIRTUAL;
#[cfg(any(feature = "v2_46", feature = "dox"))]
pub use self::constants::FILE_ATTRIBUTE_STANDARD_IS_VOLATILE;
pub use self::constants::FILE_ATTRIBUTE_STANDARD_NAME;
pub use self::constants::FILE_ATTRIBUTE_STANDARD_SIZE;
pub use self::constants::FILE_ATTRIBUTE_STANDARD_SORT_ORDER;
#[cfg(any(feature = "v2_34", feature = "dox"))]
pub use self::constants::FILE_ATTRIBUTE_STANDARD_SYMBOLIC_ICON;
pub use self::constants::FILE_ATTRIBUTE_STANDARD_SYMLINK_TARGET;
pub use self::constants::FILE_ATTRIBUTE_STANDARD_TARGET_URI;
pub use self::constants::FILE_ATTRIBUTE_STANDARD_TYPE;
pub use self::constants::FILE_ATTRIBUTE_THUMBNAILING_FAILED;
#[cfg(any(feature = "v2_40", feature = "dox"))]
pub use self::constants::FILE_ATTRIBUTE_THUMBNAIL_IS_VALID;
pub use self::constants::FILE_ATTRIBUTE_THUMBNAIL_PATH;
pub use self::constants::FILE_ATTRIBUTE_TIME_ACCESS;
pub use self::constants::FILE_ATTRIBUTE_TIME_ACCESS_USEC;
pub use self::constants::FILE_ATTRIBUTE_TIME_CHANGED;
pub use self::constants::FILE_ATTRIBUTE_TIME_CHANGED_USEC;
pub use self::constants::FILE_ATTRIBUTE_TIME_CREATED;
pub use self::constants::FILE_ATTRIBUTE_TIME_CREATED_USEC;
pub use self::constants::FILE_ATTRIBUTE_TIME_MODIFIED;
pub use self::constants::FILE_ATTRIBUTE_TIME_MODIFIED_USEC;
pub use self::constants::FILE_ATTRIBUTE_TRASH_DELETION_DATE;
pub use self::constants::FILE_ATTRIBUTE_TRASH_ITEM_COUNT;
pub use self::constants::FILE_ATTRIBUTE_TRASH_ORIG_PATH;
pub use self::constants::FILE_ATTRIBUTE_UNIX_BLOCKS;
pub use self::constants::FILE_ATTRIBUTE_UNIX_BLOCK_SIZE;
pub use self::constants::FILE_ATTRIBUTE_UNIX_DEVICE;
pub use self::constants::FILE_ATTRIBUTE_UNIX_GID;
pub use self::constants::FILE_ATTRIBUTE_UNIX_INODE;
pub use self::constants::FILE_ATTRIBUTE_UNIX_IS_MOUNTPOINT;
pub use self::constants::FILE_ATTRIBUTE_UNIX_MODE;
pub use self::constants::FILE_ATTRIBUTE_UNIX_NLINK;
pub use self::constants::FILE_ATTRIBUTE_UNIX_RDEV;
pub use self::constants::FILE_ATTRIBUTE_UNIX_UID;
pub use self::constants::MENU_ATTRIBUTE_ACTION;
#[cfg(any(feature = "v2_36", feature = "dox"))]
pub use self::constants::MENU_ATTRIBUTE_ACTION_NAMESPACE;
#[cfg(any(feature = "v2_38", feature = "dox"))]
pub use self::constants::MENU_ATTRIBUTE_ICON;
pub use self::constants::MENU_ATTRIBUTE_LABEL;
pub use self::constants::MENU_ATTRIBUTE_TARGET;
pub use self::constants::MENU_LINK_SECTION;
pub use self::constants::MENU_LINK_SUBMENU;
pub use self::constants::NATIVE_VOLUME_MONITOR_EXTENSION_POINT_NAME;
pub use self::constants::NETWORK_MONITOR_EXTENSION_POINT_NAME;
pub use self::constants::PROXY_EXTENSION_POINT_NAME;
pub use self::constants::PROXY_RESOLVER_EXTENSION_POINT_NAME;
pub use self::constants::SETTINGS_BACKEND_EXTENSION_POINT_NAME;
pub use self::constants::TLS_BACKEND_EXTENSION_POINT_NAME;
pub use self::constants::TLS_DATABASE_PURPOSE_AUTHENTICATE_CLIENT;
pub use self::constants::TLS_DATABASE_PURPOSE_AUTHENTICATE_SERVER;
pub use self::constants::VFS_EXTENSION_POINT_NAME;
pub use self::constants::VOLUME_IDENTIFIER_KIND_CLASS;
pub use self::constants::VOLUME_IDENTIFIER_KIND_HAL_UDI;
pub use self::constants::VOLUME_IDENTIFIER_KIND_LABEL;
pub use self::constants::VOLUME_IDENTIFIER_KIND_NFS_MOUNT;
pub use self::constants::VOLUME_IDENTIFIER_KIND_UNIX_DEVICE;
pub use self::constants::VOLUME_IDENTIFIER_KIND_UUID;
pub use self::constants::VOLUME_MONITOR_EXTENSION_POINT_NAME;

#[doc(hidden)]
pub mod traits {
    pub use super::ActionExt;
    pub use super::ActionGroupExt;
    pub use super::ActionMapExt;
    pub use super::AppInfoExt;
    pub use super::AppLaunchContextExt;
    pub use super::ApplicationExt;
    pub use super::BufferedInputStreamExt;
    pub use super::BufferedOutputStreamExt;
    pub use super::CancellableExt;
    pub use super::DataInputStreamExt;
    pub use super::DataOutputStreamExt;
    pub use super::FileExt;
    pub use super::FileIOStreamExt;
    pub use super::FileInfoExt;
    pub use super::FileInputStreamExt;
    pub use super::FileOutputStreamExt;
    pub use super::FilterInputStreamExt;
    pub use super::FilterOutputStreamExt;
    pub use super::IOStreamExt;
    pub use super::IconExt;
    pub use super::InetAddressExt;
    pub use super::InetSocketAddressExt;
    pub use super::InputStreamExt;
    pub use super::MemoryInputStreamExt;
    pub use super::MemoryOutputStreamExt;
    pub use super::MenuExt;
    pub use super::MenuAttributeIterExt;
    pub use super::MenuItemExt;
    pub use super::MenuLinkIterExt;
    pub use super::MenuModelExt;
    pub use super::MountOperationExt;
    pub use super::NetworkAddressExt;
    pub use super::NetworkServiceExt;
    #[cfg(any(feature = "v2_40", feature = "dox"))]
    pub use super::NotificationExt;
    pub use super::OutputStreamExt;
    pub use super::PermissionExt;
    pub use super::ResolverExt;
    pub use super::SeekableExt;
    pub use super::SettingsExt;
    pub use super::SimpleActionExt;
    pub use super::SimpleActionGroupExt;
    pub use super::SocketExt;
    pub use super::SocketAddressExt;
    pub use super::SocketAddressEnumeratorExt;
    pub use super::SocketClientExt;
    pub use super::SocketConnectableExt;
    pub use super::SocketConnectionExt;
    pub use super::SocketListenerExt;
    pub use super::SocketServiceExt;
    pub use super::TcpConnectionExt;
    pub use super::ThemedIconExt;
    pub use super::ThreadedSocketServiceExt;
    pub use super::TlsCertificateExt;
    pub use super::TlsClientConnectionExt;
    pub use super::TlsConnectionExt;
    pub use super::TlsDatabaseExt;
    pub use super::TlsFileDatabaseExt;
    pub use super::TlsInteractionExt;
    pub use super::TlsPasswordExt;
    pub use super::TlsServerConnectionExt;
}
