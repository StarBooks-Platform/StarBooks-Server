using System;
using Microsoft.EntityFrameworkCore.Migrations;

#nullable disable

namespace StarBooks.Infrastructure.Migrations
{
    public partial class InitCreateBooksRelatedTables : Migration
    {
        protected override void Up(MigrationBuilder migrationBuilder)
        {
            migrationBuilder.CreateTable(
                name: "authors",
                columns: table => new
                {
                    AuthorId = table.Column<int>(type: "int", nullable: false)
                        .Annotation("SqlServer:Identity", "1, 1"),
                    FirstName = table.Column<string>(type: "nvarchar(max)", nullable: false),
                    LastName = table.Column<string>(type: "nvarchar(max)", nullable: false)
                },
                constraints: table =>
                {
                    table.PrimaryKey("PK_authors", x => x.AuthorId);
                });

            migrationBuilder.CreateTable(
                name: "books",
                columns: table => new
                {
                    BookId = table.Column<int>(type: "int", nullable: false)
                        .Annotation("SqlServer:Identity", "1, 1"),
                    VolumeInfo_Title = table.Column<string>(type: "nvarchar(max)", nullable: false),
                    VolumeInfo_Publisher = table.Column<string>(type: "nvarchar(max)", nullable: false),
                    VolumeInfo_PublishedDate = table.Column<DateTime>(type: "datetime2", nullable: false),
                    VolumeInfo_Description = table.Column<string>(type: "nvarchar(max)", nullable: false),
                    VolumeInfo_PageCount = table.Column<int>(type: "int", nullable: false),
                    VolumeInfo_PrintType = table.Column<string>(type: "nvarchar(max)", nullable: false),
                    VolumeInfo_AverageRating = table.Column<double>(type: "float", nullable: false),
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
                    AccessInfo_QuoteSharingAllowed = table.Column<bool>(type: "bit", nullable: false)
                },
                constraints: table =>
                {
                    table.PrimaryKey("PK_books", x => x.BookId);
                });

            migrationBuilder.CreateTable(
                name: "categories",
                columns: table => new
                {
                    CategoryId = table.Column<int>(type: "int", nullable: false)
                        .Annotation("SqlServer:Identity", "1, 1"),
                    Topic = table.Column<string>(type: "nvarchar(max)", nullable: false)
                },
                constraints: table =>
                {
                    table.PrimaryKey("PK_categories", x => x.CategoryId);
                });

            migrationBuilder.CreateTable(
                name: "AuthorModelBookModel",
                columns: table => new
                {
                    AuthorsAuthorId = table.Column<int>(type: "int", nullable: false),
                    BooksBookId = table.Column<int>(type: "int", nullable: false)
                },
                constraints: table =>
                {
                    table.PrimaryKey("PK_AuthorModelBookModel", x => new { x.AuthorsAuthorId, x.BooksBookId });
                    table.ForeignKey(
                        name: "FK_AuthorModelBookModel_authors_AuthorsAuthorId",
                        column: x => x.AuthorsAuthorId,
                        principalTable: "authors",
                        principalColumn: "AuthorId",
                        onDelete: ReferentialAction.Cascade);
                    table.ForeignKey(
                        name: "FK_AuthorModelBookModel_books_BooksBookId",
                        column: x => x.BooksBookId,
                        principalTable: "books",
                        principalColumn: "BookId",
                        onDelete: ReferentialAction.Cascade);
                });

            migrationBuilder.CreateTable(
                name: "identifiers",
                columns: table => new
                {
                    IdentifierId = table.Column<int>(type: "int", nullable: false)
                        .Annotation("SqlServer:Identity", "1, 1"),
                    Type = table.Column<string>(type: "nvarchar(max)", nullable: false),
                    Identifier = table.Column<string>(type: "nvarchar(max)", nullable: false),
                    BookId = table.Column<int>(type: "int", nullable: false)
                },
                constraints: table =>
                {
                    table.PrimaryKey("PK_identifiers", x => x.IdentifierId);
                    table.ForeignKey(
                        name: "FK_identifiers_books_BookId",
                        column: x => x.BookId,
                        principalTable: "books",
                        principalColumn: "BookId",
                        onDelete: ReferentialAction.Cascade);
                });

            migrationBuilder.CreateTable(
                name: "BookModelCategoryModel",
                columns: table => new
                {
                    BooksBookId = table.Column<int>(type: "int", nullable: false),
                    CategoriesCategoryId = table.Column<int>(type: "int", nullable: false)
                },
                constraints: table =>
                {
                    table.PrimaryKey("PK_BookModelCategoryModel", x => new { x.BooksBookId, x.CategoriesCategoryId });
                    table.ForeignKey(
                        name: "FK_BookModelCategoryModel_books_BooksBookId",
                        column: x => x.BooksBookId,
                        principalTable: "books",
                        principalColumn: "BookId",
                        onDelete: ReferentialAction.Cascade);
                    table.ForeignKey(
                        name: "FK_BookModelCategoryModel_categories_CategoriesCategoryId",
                        column: x => x.CategoriesCategoryId,
                        principalTable: "categories",
                        principalColumn: "CategoryId",
                        onDelete: ReferentialAction.Cascade);
                });

            migrationBuilder.CreateIndex(
                name: "IX_AuthorModelBookModel_BooksBookId",
                table: "AuthorModelBookModel",
                column: "BooksBookId");

            migrationBuilder.CreateIndex(
                name: "IX_BookModelCategoryModel_CategoriesCategoryId",
                table: "BookModelCategoryModel",
                column: "CategoriesCategoryId");

            migrationBuilder.CreateIndex(
                name: "IX_identifiers_BookId",
                table: "identifiers",
                column: "BookId");
        }

        protected override void Down(MigrationBuilder migrationBuilder)
        {
            migrationBuilder.DropTable(
                name: "AuthorModelBookModel");

            migrationBuilder.DropTable(
                name: "BookModelCategoryModel");

            migrationBuilder.DropTable(
                name: "identifiers");

            migrationBuilder.DropTable(
                name: "authors");

            migrationBuilder.DropTable(
                name: "categories");

            migrationBuilder.DropTable(
                name: "books");
        }
    }
}
