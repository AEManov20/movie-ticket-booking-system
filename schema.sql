USE [master]
GO
/****** Object:  Database [MovieTicketBooking]    Script Date: 17.5.2023 г. 22:19:55 ******/
CREATE DATABASE [MovieTicketBooking]
 CONTAINMENT = NONE
 ON  PRIMARY 
( NAME = N'MovieTicketBooking', FILENAME = N'C:\Program Files\Microsoft SQL Server\MSSQL16.SQLEXPRESS\MSSQL\DATA\MovieTicketBooking.mdf' , SIZE = 8192KB , MAXSIZE = UNLIMITED, FILEGROWTH = 65536KB )
 LOG ON 
( NAME = N'MovieTicketBooking_log', FILENAME = N'C:\Program Files\Microsoft SQL Server\MSSQL16.SQLEXPRESS\MSSQL\DATA\MovieTicketBooking_log.ldf' , SIZE = 8192KB , MAXSIZE = 2048GB , FILEGROWTH = 65536KB )
 WITH CATALOG_COLLATION = DATABASE_DEFAULT, LEDGER = OFF
GO
ALTER DATABASE [MovieTicketBooking] SET COMPATIBILITY_LEVEL = 160
GO
IF (1 = FULLTEXTSERVICEPROPERTY('IsFullTextInstalled'))
begin
EXEC [MovieTicketBooking].[dbo].[sp_fulltext_database] @action = 'enable'
end
GO
ALTER DATABASE [MovieTicketBooking] SET ANSI_NULL_DEFAULT OFF 
GO
ALTER DATABASE [MovieTicketBooking] SET ANSI_NULLS OFF 
GO
ALTER DATABASE [MovieTicketBooking] SET ANSI_PADDING OFF 
GO
ALTER DATABASE [MovieTicketBooking] SET ANSI_WARNINGS OFF 
GO
ALTER DATABASE [MovieTicketBooking] SET ARITHABORT OFF 
GO
ALTER DATABASE [MovieTicketBooking] SET AUTO_CLOSE OFF 
GO
ALTER DATABASE [MovieTicketBooking] SET AUTO_SHRINK OFF 
GO
ALTER DATABASE [MovieTicketBooking] SET AUTO_UPDATE_STATISTICS ON 
GO
ALTER DATABASE [MovieTicketBooking] SET CURSOR_CLOSE_ON_COMMIT OFF 
GO
ALTER DATABASE [MovieTicketBooking] SET CURSOR_DEFAULT  GLOBAL 
GO
ALTER DATABASE [MovieTicketBooking] SET CONCAT_NULL_YIELDS_NULL OFF 
GO
ALTER DATABASE [MovieTicketBooking] SET NUMERIC_ROUNDABORT OFF 
GO
ALTER DATABASE [MovieTicketBooking] SET QUOTED_IDENTIFIER OFF 
GO
ALTER DATABASE [MovieTicketBooking] SET RECURSIVE_TRIGGERS OFF 
GO
ALTER DATABASE [MovieTicketBooking] SET  DISABLE_BROKER 
GO
ALTER DATABASE [MovieTicketBooking] SET AUTO_UPDATE_STATISTICS_ASYNC OFF 
GO
ALTER DATABASE [MovieTicketBooking] SET DATE_CORRELATION_OPTIMIZATION OFF 
GO
ALTER DATABASE [MovieTicketBooking] SET TRUSTWORTHY OFF 
GO
ALTER DATABASE [MovieTicketBooking] SET ALLOW_SNAPSHOT_ISOLATION OFF 
GO
ALTER DATABASE [MovieTicketBooking] SET PARAMETERIZATION SIMPLE 
GO
ALTER DATABASE [MovieTicketBooking] SET READ_COMMITTED_SNAPSHOT OFF 
GO
ALTER DATABASE [MovieTicketBooking] SET HONOR_BROKER_PRIORITY OFF 
GO
ALTER DATABASE [MovieTicketBooking] SET RECOVERY SIMPLE 
GO
ALTER DATABASE [MovieTicketBooking] SET  MULTI_USER 
GO
ALTER DATABASE [MovieTicketBooking] SET PAGE_VERIFY CHECKSUM  
GO
ALTER DATABASE [MovieTicketBooking] SET DB_CHAINING OFF 
GO
ALTER DATABASE [MovieTicketBooking] SET FILESTREAM( NON_TRANSACTED_ACCESS = OFF ) 
GO
ALTER DATABASE [MovieTicketBooking] SET TARGET_RECOVERY_TIME = 60 SECONDS 
GO
ALTER DATABASE [MovieTicketBooking] SET DELAYED_DURABILITY = DISABLED 
GO
ALTER DATABASE [MovieTicketBooking] SET ACCELERATED_DATABASE_RECOVERY = OFF  
GO
ALTER DATABASE [MovieTicketBooking] SET QUERY_STORE = ON
GO
ALTER DATABASE [MovieTicketBooking] SET QUERY_STORE (OPERATION_MODE = READ_WRITE, CLEANUP_POLICY = (STALE_QUERY_THRESHOLD_DAYS = 30), DATA_FLUSH_INTERVAL_SECONDS = 900, INTERVAL_LENGTH_MINUTES = 60, MAX_STORAGE_SIZE_MB = 1000, QUERY_CAPTURE_MODE = AUTO, SIZE_BASED_CLEANUP_MODE = AUTO, MAX_PLANS_PER_QUERY = 200, WAIT_STATS_CAPTURE_MODE = ON)
GO
USE [MovieTicketBooking]
GO
/****** Object:  Table [dbo].[ExternalCredentials]    Script Date: 17.5.2023 г. 22:19:55 ******/
SET ANSI_NULLS ON
GO
SET QUOTED_IDENTIFIER ON
GO
CREATE TABLE [dbo].[ExternalCredentials](
	[Id] [int] NOT NULL,
	[Provider] [varchar](50) NOT NULL,
	[ExternalId] [varchar](150) NOT NULL,
	[UserId] [int] NOT NULL,
 CONSTRAINT [PK_ExternalCredentials] PRIMARY KEY CLUSTERED 
(
	[Id] ASC
)WITH (PAD_INDEX = OFF, STATISTICS_NORECOMPUTE = OFF, IGNORE_DUP_KEY = OFF, ALLOW_ROW_LOCKS = ON, ALLOW_PAGE_LOCKS = ON, OPTIMIZE_FOR_SEQUENTIAL_KEY = OFF) ON [PRIMARY]
) ON [PRIMARY]
GO
/****** Object:  Table [dbo].[Halls]    Script Date: 17.5.2023 г. 22:19:55 ******/
SET ANSI_NULLS ON
GO
SET QUOTED_IDENTIFIER ON
GO
CREATE TABLE [dbo].[Halls](
	[Id] [int] NOT NULL,
	[Number] [int] NOT NULL,
	[TheatreId] [int] NOT NULL,
	[SeatData] [xml] NULL,
 CONSTRAINT [PK_Halls] PRIMARY KEY CLUSTERED 
(
	[Id] ASC
)WITH (PAD_INDEX = OFF, STATISTICS_NORECOMPUTE = OFF, IGNORE_DUP_KEY = OFF, ALLOW_ROW_LOCKS = ON, ALLOW_PAGE_LOCKS = ON, OPTIMIZE_FOR_SEQUENTIAL_KEY = OFF) ON [PRIMARY]
) ON [PRIMARY] TEXTIMAGE_ON [PRIMARY]
GO
/****** Object:  Table [dbo].[MovieReviews]    Script Date: 17.5.2023 г. 22:19:55 ******/
SET ANSI_NULLS ON
GO
SET QUOTED_IDENTIFIER ON
GO
CREATE TABLE [dbo].[MovieReviews](
	[Id] [int] NOT NULL,
	[AuthorUserId] [int] NOT NULL,
	[MovieId] [int] NOT NULL,
	[Content] [nvarchar](max) NULL,
	[Rating] [float] NOT NULL,
 CONSTRAINT [PK_MovieReviews] PRIMARY KEY CLUSTERED 
(
	[Id] ASC
)WITH (PAD_INDEX = OFF, STATISTICS_NORECOMPUTE = OFF, IGNORE_DUP_KEY = OFF, ALLOW_ROW_LOCKS = ON, ALLOW_PAGE_LOCKS = ON, OPTIMIZE_FOR_SEQUENTIAL_KEY = OFF) ON [PRIMARY]
) ON [PRIMARY] TEXTIMAGE_ON [PRIMARY]
GO
/****** Object:  Table [dbo].[Movies]    Script Date: 17.5.2023 г. 22:19:55 ******/
SET ANSI_NULLS ON
GO
SET QUOTED_IDENTIFIER ON
GO
CREATE TABLE [dbo].[Movies](
	[Id] [int] NOT NULL,
	[Name] [nvarchar](50) NOT NULL,
	[Description] [nvarchar](50) NOT NULL,
	[Genre] [varchar](50) NOT NULL,
	[ReleaseDate] [date] NOT NULL,
	[Length] [float] NOT NULL,
	[ImdbLink] [varchar](250) NULL,
 CONSTRAINT [PK_Movies] PRIMARY KEY CLUSTERED 
(
	[Id] ASC
)WITH (PAD_INDEX = OFF, STATISTICS_NORECOMPUTE = OFF, IGNORE_DUP_KEY = OFF, ALLOW_ROW_LOCKS = ON, ALLOW_PAGE_LOCKS = ON, OPTIMIZE_FOR_SEQUENTIAL_KEY = OFF) ON [PRIMARY]
) ON [PRIMARY]
GO
/****** Object:  Table [dbo].[TheatreMovies]    Script Date: 17.5.2023 г. 22:19:55 ******/
SET ANSI_NULLS ON
GO
SET QUOTED_IDENTIFIER ON
GO
CREATE TABLE [dbo].[TheatreMovies](
	[Id] [int] NOT NULL,
	[MovieId] [int] NOT NULL,
	[HallId] [int] NOT NULL,
	[SubtitlesLanguage] [varchar](50) NULL,
	[AudioLanguage] [varchar](50) NOT NULL,
	[StartingTime] [date] NOT NULL,
 CONSTRAINT [PK_TheatreMovies] PRIMARY KEY CLUSTERED 
(
	[Id] ASC
)WITH (PAD_INDEX = OFF, STATISTICS_NORECOMPUTE = OFF, IGNORE_DUP_KEY = OFF, ALLOW_ROW_LOCKS = ON, ALLOW_PAGE_LOCKS = ON, OPTIMIZE_FOR_SEQUENTIAL_KEY = OFF) ON [PRIMARY]
) ON [PRIMARY]
GO
/****** Object:  Table [dbo].[TheatrePermissions]    Script Date: 17.5.2023 г. 22:19:55 ******/
SET ANSI_NULLS ON
GO
SET QUOTED_IDENTIFIER ON
GO
CREATE TABLE [dbo].[TheatrePermissions](
	[Id] [int] NOT NULL,
	[UserId] [int] NOT NULL,
	[TheatreId] [int] NOT NULL,
	[CanManageUsers] [bit] NOT NULL,
	[CanManageMovies] [bit] NOT NULL,
	[CanCheckTickets] [bit] NOT NULL,
	[CanManageTickets] [bit] NOT NULL
) ON [PRIMARY]
GO
/****** Object:  Table [dbo].[Theatres]    Script Date: 17.5.2023 г. 22:19:55 ******/
SET ANSI_NULLS ON
GO
SET QUOTED_IDENTIFIER ON
GO
CREATE TABLE [dbo].[Theatres](
	[Id] [int] NOT NULL,
	[Name] [nvarchar](50) NOT NULL,
	[Location] [geography] NOT NULL,
 CONSTRAINT [PK_Theatres] PRIMARY KEY CLUSTERED 
(
	[Id] ASC
)WITH (PAD_INDEX = OFF, STATISTICS_NORECOMPUTE = OFF, IGNORE_DUP_KEY = OFF, ALLOW_ROW_LOCKS = ON, ALLOW_PAGE_LOCKS = ON, OPTIMIZE_FOR_SEQUENTIAL_KEY = OFF) ON [PRIMARY]
) ON [PRIMARY] TEXTIMAGE_ON [PRIMARY]
GO
/****** Object:  Table [dbo].[Tickets]    Script Date: 17.5.2023 г. 22:19:55 ******/
SET ANSI_NULLS ON
GO
SET QUOTED_IDENTIFIER ON
GO
CREATE TABLE [dbo].[Tickets](
	[Id] [int] NOT NULL,
	[OwnerUserId] [int] NOT NULL,
	[TheatreMovieId] [int] NOT NULL,
	[SeatRow] [int] NOT NULL,
	[SeatColumn] [int] NOT NULL,
	[AccessKey] [uniqueidentifier] NOT NULL,
	[Used] [bit] NOT NULL
) ON [PRIMARY]
GO
/****** Object:  Table [dbo].[Users]    Script Date: 17.5.2023 г. 22:19:55 ******/
SET ANSI_NULLS ON
GO
SET QUOTED_IDENTIFIER ON
GO
CREATE TABLE [dbo].[Users](
	[Id] [int] NOT NULL,
	[FirstName] [nvarchar](50) NOT NULL,
	[LastName] [nvarchar](50) NOT NULL,
	[Email] [varchar](150) NOT NULL,
	[Username] [varchar](50) NOT NULL,
	[IsSuperUser] [bit] NOT NULL,
 CONSTRAINT [PK_Users] PRIMARY KEY CLUSTERED 
(
	[Id] ASC
)WITH (PAD_INDEX = OFF, STATISTICS_NORECOMPUTE = OFF, IGNORE_DUP_KEY = OFF, ALLOW_ROW_LOCKS = ON, ALLOW_PAGE_LOCKS = ON, OPTIMIZE_FOR_SEQUENTIAL_KEY = OFF) ON [PRIMARY]
) ON [PRIMARY]
GO
ALTER TABLE [dbo].[TheatrePermissions] ADD  CONSTRAINT [DF_TheatrePermissions_CanManageUsers]  DEFAULT ((0)) FOR [CanManageUsers]
GO
ALTER TABLE [dbo].[TheatrePermissions] ADD  CONSTRAINT [DF_TheatrePermissions_CanManageMovies]  DEFAULT ((0)) FOR [CanManageMovies]
GO
ALTER TABLE [dbo].[TheatrePermissions] ADD  CONSTRAINT [DF_TheatrePermissions_CanCheckTickets]  DEFAULT ((0)) FOR [CanCheckTickets]
GO
ALTER TABLE [dbo].[TheatrePermissions] ADD  CONSTRAINT [DF_TheatrePermissions_CanManageTickets]  DEFAULT ((0)) FOR [CanManageTickets]
GO
ALTER TABLE [dbo].[Tickets] ADD  CONSTRAINT [DF_Tickets_AccessKey]  DEFAULT (CONVERT([uniqueidentifier],Crypt_Gen_Random((16)))) FOR [AccessKey]
GO
ALTER TABLE [dbo].[Tickets] ADD  CONSTRAINT [DF_Tickets_Used]  DEFAULT ((0)) FOR [Used]
GO
ALTER TABLE [dbo].[Users] ADD  CONSTRAINT [DF_Users_IsSuperUser]  DEFAULT ((0)) FOR [IsSuperUser]
GO
ALTER TABLE [dbo].[ExternalCredentials]  WITH CHECK ADD  CONSTRAINT [FK_ExternalCredentials_Users] FOREIGN KEY([UserId])
REFERENCES [dbo].[Users] ([Id])
GO
ALTER TABLE [dbo].[ExternalCredentials] CHECK CONSTRAINT [FK_ExternalCredentials_Users]
GO
ALTER TABLE [dbo].[Halls]  WITH CHECK ADD  CONSTRAINT [FK_Halls_Theatres] FOREIGN KEY([TheatreId])
REFERENCES [dbo].[Theatres] ([Id])
GO
ALTER TABLE [dbo].[Halls] CHECK CONSTRAINT [FK_Halls_Theatres]
GO
ALTER TABLE [dbo].[MovieReviews]  WITH CHECK ADD  CONSTRAINT [FK_MovieReviews_Movies] FOREIGN KEY([MovieId])
REFERENCES [dbo].[Movies] ([Id])
GO
ALTER TABLE [dbo].[MovieReviews] CHECK CONSTRAINT [FK_MovieReviews_Movies]
GO
ALTER TABLE [dbo].[MovieReviews]  WITH CHECK ADD  CONSTRAINT [FK_MovieReviews_Users] FOREIGN KEY([AuthorUserId])
REFERENCES [dbo].[Users] ([Id])
GO
ALTER TABLE [dbo].[MovieReviews] CHECK CONSTRAINT [FK_MovieReviews_Users]
GO
ALTER TABLE [dbo].[TheatreMovies]  WITH CHECK ADD  CONSTRAINT [FK_TheatreMovies_Halls] FOREIGN KEY([HallId])
REFERENCES [dbo].[Halls] ([Id])
GO
ALTER TABLE [dbo].[TheatreMovies] CHECK CONSTRAINT [FK_TheatreMovies_Halls]
GO
ALTER TABLE [dbo].[TheatreMovies]  WITH CHECK ADD  CONSTRAINT [FK_TheatreMovies_Movies] FOREIGN KEY([MovieId])
REFERENCES [dbo].[Movies] ([Id])
GO
ALTER TABLE [dbo].[TheatreMovies] CHECK CONSTRAINT [FK_TheatreMovies_Movies]
GO
ALTER TABLE [dbo].[TheatrePermissions]  WITH CHECK ADD  CONSTRAINT [FK_TheatrePermissions_Theatres] FOREIGN KEY([TheatreId])
REFERENCES [dbo].[Theatres] ([Id])
GO
ALTER TABLE [dbo].[TheatrePermissions] CHECK CONSTRAINT [FK_TheatrePermissions_Theatres]
GO
ALTER TABLE [dbo].[TheatrePermissions]  WITH CHECK ADD  CONSTRAINT [FK_TheatrePermissions_Users] FOREIGN KEY([UserId])
REFERENCES [dbo].[Users] ([Id])
GO
ALTER TABLE [dbo].[TheatrePermissions] CHECK CONSTRAINT [FK_TheatrePermissions_Users]
GO
ALTER TABLE [dbo].[Tickets]  WITH CHECK ADD  CONSTRAINT [FK_Tickets_TheatreMovies] FOREIGN KEY([TheatreMovieId])
REFERENCES [dbo].[TheatreMovies] ([Id])
GO
ALTER TABLE [dbo].[Tickets] CHECK CONSTRAINT [FK_Tickets_TheatreMovies]
GO
ALTER TABLE [dbo].[Tickets]  WITH CHECK ADD  CONSTRAINT [FK_Tickets_Users] FOREIGN KEY([OwnerUserId])
REFERENCES [dbo].[Users] ([Id])
GO
ALTER TABLE [dbo].[Tickets] CHECK CONSTRAINT [FK_Tickets_Users]
GO
USE [master]
GO
ALTER DATABASE [MovieTicketBooking] SET  READ_WRITE 
GO
