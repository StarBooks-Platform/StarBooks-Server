using System;
using Microsoft.EntityFrameworkCore.Migrations;

#nullable disable

namespace StarBooks.Infrastructure.Migrations
{
    public partial class AddManyToManyBetweenAuthor_Category_Identifier_And_Book : Migration
    {
        protected override void Up(MigrationBuilder migrationBuilder)
        {
            migrationBuilder.DropForeignKey(
                name: "FK_books_authors_AuthorModelId",
                table: "books");

            migrationBuilder.DropIndex(
                name: "IX_books_AuthorModelId",
                table: "books");

            migrationBuilder.DropColumn(
                name: "AuthorModelId",
                table: "books");

            migrationBuilder.AddColumn<Guid>(
                name: "BookModelId",
                table: "identifiers",
                type: "uniqueidentifier",
                nullable: true);

            migrationBuilder.AddColumn<Guid>(
                name: "BookModelId",
                table: "categories",
                type: "uniqueidentifier",
                nullable: true);

            migrationBuilder.CreateTable(
                name: "AuthorModelBookModel",
                columns: table => new
                {
                    AuthorsId = table.Column<Guid>(type: "uniqueidentifier", nullable: false),
                    BooksId = table.Column<Guid>(type: "uniqueidentifier", nullable: false)
                },
                constraints: table =>
                {
                    table.PrimaryKey("PK_AuthorModelBookModel", x => new { x.AuthorsId, x.BooksId });
                    table.ForeignKey(
                        name: "FK_AuthorModelBookModel_authors_AuthorsId",
                        column: x => x.AuthorsId,
                        principalTable: "authors",
                        principalColumn: "Id",
                        onDelete: ReferentialAction.Cascade);
                    table.ForeignKey(
                        name: "FK_AuthorModelBookModel_books_BooksId",
                        column: x => x.BooksId,
                        principalTable: "books",
                        principalColumn: "Id",
                        onDelete: ReferentialAction.Cascade);
                });

            migrationBuilder.CreateIndex(
                name: "IX_identifiers_BookModelId",
                table: "identifiers",
                column: "BookModelId");

            migrationBuilder.CreateIndex(
                name: "IX_categories_BookModelId",
                table: "categories",
                column: "BookModelId");

            migrationBuilder.CreateIndex(
                name: "IX_AuthorModelBookModel_BooksId",
                table: "AuthorModelBookModel",
                column: "BooksId");

            migrationBuilder.AddForeignKey(
                name: "FK_categories_books_BookModelId",
                table: "categories",
                column: "BookModelId",
                principalTable: "books",
                principalColumn: "Id");

            migrationBuilder.AddForeignKey(
                name: "FK_identifiers_books_BookModelId",
                table: "identifiers",
                column: "BookModelId",
                principalTable: "books",
                principalColumn: "Id");
        }

        protected override void Down(MigrationBuilder migrationBuilder)
        {
            migrationBuilder.DropForeignKey(
                name: "FK_categories_books_BookModelId",
                table: "categories");

            migrationBuilder.DropForeignKey(
                name: "FK_identifiers_books_BookModelId",
                table: "identifiers");

            migrationBuilder.DropTable(
                name: "AuthorModelBookModel");

            migrationBuilder.DropIndex(
                name: "IX_identifiers_BookModelId",
                table: "identifiers");

            migrationBuilder.DropIndex(
                name: "IX_categories_BookModelId",
                table: "categories");

            migrationBuilder.DropColumn(
                name: "BookModelId",
                table: "identifiers");

            migrationBuilder.DropColumn(
                name: "BookModelId",
                table: "categories");

            migrationBuilder.AddColumn<Guid>(
                name: "AuthorModelId",
                table: "books",
                type: "uniqueidentifier",
                nullable: true);

            migrationBuilder.CreateIndex(
                name: "IX_books_AuthorModelId",
                table: "books",
                column: "AuthorModelId");

            migrationBuilder.AddForeignKey(
                name: "FK_books_authors_AuthorModelId",
                table: "books",
                column: "AuthorModelId",
                principalTable: "authors",
                principalColumn: "Id");
        }
    }
}
