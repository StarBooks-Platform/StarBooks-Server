using System;
using Microsoft.EntityFrameworkCore.Migrations;

#nullable disable

namespace StarBooks.Infrastructure.Migrations
{
    public partial class InitialCreateBooksTable : Migration
    {
        protected override void Up(MigrationBuilder migrationBuilder)
        {
            migrationBuilder.CreateTable(
                name: "authors",
                columns: table => new
                {
                    Id = table.Column<Guid>(type: "uniqueidentifier", nullable: false),
                    FirstName = table.Column<string>(type: "nvarchar(max)", nullable: false),
                    LastName = table.Column<string>(type: "nvarchar(max)", nullable: false)
                },
                constraints: table =>
                {
                    table.PrimaryKey("PK_authors", x => x.Id);
                });

            migrationBuilder.CreateTable(
                name: "categories",
                columns: table => new
                {
                    Id = table.Column<Guid>(type: "uniqueidentifier", nullable: false),
                    Topic = table.Column<string>(type: "nvarchar(max)", nullable: false)
                },
                constraints: table =>
                {
                    table.PrimaryKey("PK_categories", x => x.Id);
                });

            migrationBuilder.CreateTable(
                name: "identifiers",
                columns: table => new
                {
                    Id = table.Column<Guid>(type: "uniqueidentifier", nullable: false),
                    Type = table.Column<string>(type: "nvarchar(max)", nullable: false),
                    Identifier = table.Column<string>(type: "nvarchar(max)", nullable: false)
                },
                constraints: table =>
                {
                    table.PrimaryKey("PK_identifiers", x => x.Id);
                });

            migrationBuilder.CreateTable(
                name: "books",
                columns: table => new
                {
                    Id = table.Column<Guid>(type: "uniqueidentifier", nullable: false),
                    VolumeInfo_Title = table.Column<string>(type: "nvarchar(max)", nullable: false),
                    VolumeInfo_Publisher = table.Column<string>(type: "nvarchar(max)", nullable: false),
                    VolumeInfo_PublishedDate = table.Column<string>(type: "nvarchar(max)", nullable: false),
                    VolumeInfo_Description = table.Column<string>(type: "nvarchar(max)", nullable: false),
                    VolumeInfo_PageCount = table.Column<int>(type: "int", nullable: false),
                    VolumeInfo_PrintType = table.Column<string>(type: "nvarchar(max)", nullable: false),
                    VolumeInfo_AverageRating = table.Column<decimal>(type: "decimal(18,2)", nullable: false),
                    VolumeInfo_RatingsCount = table.Column<int>(type: "int", nullable: false),
                    VolumeInfo_ImageLinks_ThumbnailUrl = table.Column<string>(type: "nvarchar(max)", nullable: false),
                    VolumeInfo_Language = table.Column<string>(type: "nvarchar(max)", nullable: false),
                    VolumeInfo_PreviewLink = table.Column<string>(type: "nvarchar(max)", nullable: false),
                    VolumeInfo_InfoLink = table.Column<string>(type: "nvarchar(max)", nullable: false),
                    SaleInfo_Country = table.Column<string>(type: "nvarchar(max)", nullable: false),
                    SaleInfo_IsEbook = table.Column<bool>(type: "bit", nullable: false),
                    AccessInfo_Country = table.Column<string>(type: "nvarchar(max)", nullable: false),
                    AccessInfo_Viewability = table.Column<string>(type: "nvarchar(max)", nullable: false),
                    AccessInfo_Embeddable = table.Column<bool>(type: "bit", nullable: false),
                    AccessInfo_PublicDomain = table.Column<bool>(type: "bit", nullable: false),
                    AccessInfo_TextToSpeechPermission = table.Column<string>(type: "nvarchar(max)", nullable: false),
                    AccessInfo_Epub_IsAvailable = table.Column<bool>(type: "bit", nullable: false),
                    AccessInfo_Epub_AcsTokenLink = table.Column<string>(type: "nvarchar(max)", nullable: false),
                    AccessInfo_Pdf_IsAvailable = table.Column<bool>(type: "bit", nullable: false),
                    AccessInfo_Pdf_AcsTokenLink = table.Column<string>(type: "nvarchar(max)", nullable: false),
                    AccessInfo_WebReaderLink = table.Column<string>(type: "nvarchar(max)", nullable: false),
                    AccessInfo_QuoteSharingAllowed = table.Column<bool>(type: "bit", nullable: false),
                    AuthorModelId = table.Column<Guid>(type: "uniqueidentifier", nullable: true)
                },
                constraints: table =>
                {
                    table.PrimaryKey("PK_books", x => x.Id);
                    table.ForeignKey(
                        name: "FK_books_authors_AuthorModelId",
                        column: x => x.AuthorModelId,
                        principalTable: "authors",
                        principalColumn: "Id");
                });

            migrationBuilder.CreateIndex(
                name: "IX_books_AuthorModelId",
                table: "books",
                column: "AuthorModelId");
        }

        protected override void Down(MigrationBuilder migrationBuilder)
        {
            migrationBuilder.DropTable(
                name: "books");

            migrationBuilder.DropTable(
                name: "categories");

            migrationBuilder.DropTable(
                name: "identifiers");

            migrationBuilder.DropTable(
                name: "authors");
        }
    }
}
